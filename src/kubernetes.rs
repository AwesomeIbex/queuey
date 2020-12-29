use std::path::PathBuf;

use anyhow::{Context, Error, anyhow};
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{Api, ListParams, Meta, PatchParams, PostParams, WatchEvent};
use kube::Client;
use kube::error::ErrorResponse;
use serde_json::{Error as SerdeError, Value};

use crate::cli::Opts;

const WORKY_DEPLOYMENT: &'static str = "worky";

pub async fn create_workers(opts: &Opts) -> Result<i32, Error> {
    let client = Client::try_default().await?;
    let deployments: Api<Deployment> = Api::namespaced(client, "default");
    let mut should_watch = true;
    match deployments.get(WORKY_DEPLOYMENT).await {
        Ok(exists) => {
            log::trace!("Deployment already exists, checking scale..");
            let deployment_replicas = opts.workers;
            if exists
                .spec.context("Spec does not exist")?
                .replicas.context("Replicas do not exist")? == deployment_replicas {
                log::info!("Has the right amount of replicas, not scaling..");
                should_watch = false;
            } else {
                log::info!("Scaling to {} workers..", deployment_replicas);
                let params = PatchParams::apply(WORKY_DEPLOYMENT).force();
                let patch = serde_yaml::to_vec(&build_patch_deployment_request(&deployment_replicas))?;
                deployments.patch(WORKY_DEPLOYMENT, &params, patch).await?;
            }
        }
        Err(_) => {
            let deployment = build_deployment_request(&opts)?;
            deployments.create(&PostParams::default(), &deployment).await?;
        }
    }

    if should_watch {
        Ok(watch_deployment(deployments).await?)
    } else {
        Ok(opts.workers)
    }
}

async fn watch_deployment(deployments: Api<Deployment>) -> Result<i32, Error> {
    let params = ListParams::default()
        .fields(&format!("metadata.name={}", WORKY_DEPLOYMENT))
        .timeout(20);
    let mut stream = deployments
        .watch(&params, "0")
        .await
        .context("Failed to watch deployment")?
        .boxed();

    let mut available_result = Ok(0);
    while let Some(status) = stream.try_next().await.context("There was an error reading the next stream")? {
        match status {
            WatchEvent::Added(o) => println!("Added {}", Meta::name(&o)),
            WatchEvent::Modified(o) => {
                let status = o.status.as_ref().expect("status exists on deployment");
                let replicas = status.available_replicas.clone().unwrap_or_default();
                let unavailable = status.unavailable_replicas.clone().unwrap_or_default();
                log::trace!("Modified: {}, current available replicas: {}, unavailable: {}", Meta::name(&o), replicas, unavailable);
                available_result = Ok(replicas)
            }
            WatchEvent::Deleted(o) => log::trace!("Deleted {}", Meta::name(&o)),
            WatchEvent::Error(e) => {
                log::error!("Error {:?}", e);
                available_result = Err(anyhow!(e));
            }
            _ => log::trace!("Some status event {:?}", status)
        }
    }
    available_result
}

fn build_patch_deployment_request(deployment_replicas: &i32) -> Value {
    serde_json::json!(
        {
            "apiVersion": "apps/v1",
            "kind": "Deployment",
            "spec": {
               "replicas": deployment_replicas
            }
        }
    )
}

//TODO tidy this up better
fn build_deployment_request(opts: &Opts) -> Result<Deployment, Error> {
    serde_json::from_value(serde_json::json!({
  "kind": "Deployment",
  "spec": {
    "replicas": opts.workers,
    "template": {
      "spec": {
        "volumes" : [
            {
                "name": "queuey",
                "hostPath": {
                    "path": opts.jobs_path.to_str().context("Failed to write the jobs path as a string")?,
                    "type": "DirectoryOrCreate"
                }
            }
        ],
        "containers": [
          {
            "image": "awesomeibex/queuey:latest",
            "name": "worky",
            "volumeMounts": [
              {
                "mountPath": "/tmp/queuey",
                "name": "queuey"
              }
            ],
            "resources": {
              "requests": {
                "cpu": "64m",
                "memory": "128Mi"
              },
              "limits": {
                "cpu": "64m",
                "memory": "128Mi"
              }
            }
          }
        ],
        "initContainers": [
          {
            "image": "busybox",
            "name": "fix-volumes",
            "command": [
                "sh", "-c", "adduser -D donovand && addgroup donovand donovand && chown donovand:donovand -R /queuey"
            ],
            "volumeMounts": [
              {
                "mountPath": "/queuey",
                "name": "queuey"
              }
            ]
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
})).context("Failed to read kubernetes deployment")
}