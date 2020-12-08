use std::sync::mpsc::channel;
use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use std::time::Duration;
use crate::cli::Opts;
use std::fs;
use rand::Rng;
use rand::distributions::Alphanumeric;

mod cli;
mod dispatcher;


///
/// User will pass in a command file, this could be a raw file or a location to the file
///
/// cli could create a script from some yaml file, but for now we will just expect the script to be in the FS OR passed in raw
/// cli will read file and determine some things
/// the type of job
/// the directory to store updates and dispatch
///
fn main() {
    pretty_env_logger::init();

    let cli_opts = cli::get_opts_args();

    // Receive events
    // How?
    // Watch FS(easy over NFS, S3 and so on, lightweight, no services stood up) - notify
    // Wait over REST
    // MQ(heavyweight??, requires client side implementation, could be a module) - zmq

    // Watcher related stuff
    let (watcher_tx, watcher_rx) = channel();
    let mut watcher = watcher(watcher_tx, Duration::from_millis(300)).unwrap(); //TODO test delay
    let watcher_path = determine_path(&cli_opts.persistent);
    create_directories(&watcher_path);
    // TODO create directory of path if it doesnt exist, this will also generate worker directories maybe
    watcher.watch(&watcher_path, RecursiveMode::Recursive).unwrap();

    loop {
        match watcher_rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::NoticeWrite(_) => {} // Might need to close file access
                    DebouncedEvent::NoticeRemove(_) => {} // Same as above
                    DebouncedEvent::Create(_) => {} // Log depending on prefix of the name
                    DebouncedEvent::Write(_) => {} // Log depending on prefix of the name
                    DebouncedEvent::Chmod(_) => {} // Could be smart depending on permissions maybe
                    DebouncedEvent::Remove(_) => {} // Job is finished, store that elsewhere
                    DebouncedEvent::Rename(_, _) => {} // Job is finished, store that elsewhere
                    DebouncedEvent::Rescan => {} // Log
                    DebouncedEvent::Error(_, _) => {} // Log
                }
                dispatcher::dispatch(event)
            },
            Err(e) => dispatcher::dispatch_error(e),
        }
    }


    // Notes
    //
    // Might require a separate event loop to dispatch events(generally clean anyway)
    //
}

fn determine_path(persistent: &bool) -> String {
    if *persistent {
        String::from("./script_results")
    } else {
        let mut placeholder = String::from("/tmp/worky_");
        let mut file_name = rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>();
        placeholder.push_str(&file_name);
        placeholder
    }
}
fn create_directories(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)
}