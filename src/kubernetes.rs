use std::path::PathBuf;

use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{Api, ListParams, Meta, PatchParams, PostParams, WatchEvent};
use kube::Client;
use serde_json::Error;

#[tokio::main]
pub async fn create_workers(rx: std::sync::mpsc::Receiver<(i32, PathBuf)>) -> Result<(), kube::Error> {
    let client = Client::try_default().await.unwrap();
    let deployments: Api<Deployment> = Api::namespaced(client, "default");
    match deployments.get("worky").await {
        Ok(exists) => {
            println!("Deployment already exists, checking scale..");
            let deployment_replicas = rx.recv().unwrap().0;
            if exists.spec.unwrap().replicas.unwrap() == deployment_replicas {
                println!("Has the right amount of replicas, not scaling..")
            } else {
                println!("Scaling to {} workers..", deployment_replicas);
                let params = PatchParams::apply("worky").force();
                let patch = serde_yaml::to_vec(&serde_json::json!({
                        "apiVersion": "apps/v1",
                        "kind": "Deployment",
                        "spec": {
                           "replicas": deployment_replicas
                        }
                    })).unwrap();
                deployments.patch("worky", &params, patch).await?;
            }
        }
        Err(_) => {
            let deployment = build_deployment_request().unwrap(); // TODO rx recv on those
            deployments.create(&PostParams::default(), &deployment).await?; // Check if it exists, if it does then we scale

            let lp = ListParams::default()
                .fields(&format!("metadata.name={}", "worky-v0.0.1"))
                .timeout(10);
            let mut stream = deployments.watch(&lp, "0").await?.boxed();

            // Observe the pods phase for 10 seconds
            while let Some(status) = stream.try_next().await? {
                match status {
                    WatchEvent::Added(o) => println!("Added {}", Meta::name(&o)),
                    WatchEvent::Modified(o) => {
                        let status = o.status.as_ref().expect("status exists on deployment");
                        let available = status.available_replicas.clone().unwrap_or_default();
                        let unavailable = status.unavailable_replicas.clone().unwrap_or_default();
                        println!("Modified: {}, current available replicas: {}, unavailable: {}", Meta::name(&o), available, unavailable);
                    }
                    WatchEvent::Deleted(o) => println!("Deleted {}", Meta::name(&o)),
                    WatchEvent::Error(e) => println!("Error {:?}", e),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn build_deployment_request() -> Result<Deployment, Error> {
    serde_json::from_value(serde_json::json!({
  "kind": "Deployment",
  "spec": {
    "replicas": 10,
    "template": {
      "spec": {
        "volumes" : [
            {
                "name": "queuey",
                "hostPath": {
                    "path": "/tmp/queuey-k8s",
                    "type": "DirectoryOrCreate"
                }
            }
        ],
        "containers": [
          {
            "image": "awesomeibex/worky:latest",
            "name": "worky",
            "volumeMounts": [
              {
                "mountPath": "/tmp/queuey",
                "name": "queuey"
              }
            ],
            "resources": {
              "requests": {
                "cpu": "80m",
                "memory": "128Mi"
              },
              "limits": {
                "cpu": "80m",
                "memory": "128Mi"
              }
            }
          }
        ]
      },
      "metadata": {
        "labels": {
          "k8s-app": "worky",
          "version": "v0.0.1"
        }
      }
    },
    "selector": {
      "matchLabels": {
        "k8s-app": "worky",
        "version": "v0.0.1"
      }
    }
  },
  "apiVersion": "apps/v1",
  "metadata": {
    "labels": {
      "k8s-app": "worky",
      "version": "v0.0.1"
    },
    "namespace": "default",
    "name": "worky"
  }
}))
}