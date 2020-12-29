use std::fs;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

use notify::{DebouncedEvent, RecursiveMode, watcher, Watcher};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::dispatcher::DispatchType;
use crate::cli::Platform;

mod cli;
mod dispatcher;
mod kubernetes;

///
/// User will pass in a command file, this could be a raw file or a location to the file
///
/// cli could create a script from some yaml file, but for now we will just expect the script to be in the FS OR passed in raw
/// cli will read file and determine some things
/// the type of job
/// the directory to store updates and dispatch
///
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cli_opts = cli::get_opts_args();
    let job_id = get_random_job_id();

    match cli_opts.platform {
        Platform::Local => {}
        Platform::Kubernetes => {
            if let Err(err) = kubernetes::create_workers(&cli_opts).await {
                log::error!("Failed to create workers: {}", err)
            }
        }
    }

    let (watcher_tx, watcher_rx) = channel();
    let mut watcher = watcher(watcher_tx, Duration::from_millis(300)).unwrap(); //TODO test delay
    fs::create_dir_all(&cli_opts.jobs_path);
    watcher.watch(&cli_opts.jobs_path, RecursiveMode::Recursive).unwrap();

    dispatcher::dispatch(&job_id, &cli_opts, DispatchType::FILE); //TODO might param this but for now its file
    loop {
        match watcher_rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path) => {
                        log::trace!("A file was created at path: {:?}", path)
                    } // Log depending on prefix of the name OR could use permissions
                    DebouncedEvent::Write(_) => {} // Log depending on prefix of the name
                    DebouncedEvent::Remove(path) => {
                        log::trace!("A worker deregistered, path: {:?}", path)
                    }
                    DebouncedEvent::Rename(_, _) => {} // Job is finished, store that elsewhere
                    DebouncedEvent::Error(err, path) => {
                        log::error!("There was an error at path {:?}, {}", path, err)
                    } // Log
                    _ => {
                        log::trace!("An event was triggered {:?}", event)
                    }
                }
            }
            Err(e) => dispatcher::dispatch_error(e),
        }
    }


    // Notes
    //
    // Might require a separate event loop to dispatch events(generally clean anyway)
    //
}

fn get_random_job_id() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>()
}