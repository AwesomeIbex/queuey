use std::sync::mpsc::channel;
use notify::{watcher, RecursiveMode, Watcher};
use std::time::Duration;

mod cli;

fn main() {
    pretty_env_logger::init();

    let cli_opts = cli::get_opts_args();

    /// Receive events
    /// How?
    /// Watch FS(easy over NFS, S3 and so on, lightweight, no services stood up) - notify
    /// Wait over REST
    /// MQ(heavyweight??, requires client side implementation, could be a module) - zmq

    // Watcher related stuff
    let (watcher_tx, watcher_rx) = channel();
    let mut watcher = watcher(watcher_tx, Duration::from_millis(300)).unwrap(); //TODO test delay
    watcher.watch(cli_opts.file, RecursiveMode::Recursive).unwrap();

    loop {
        match watcher_rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    // Dispatch events(dispatcher)
    /// How might one dispatch events?
    /// FS(folders, worker prefix, worker then implements same watch interface)
    /// REST(post to workers that we know about, easy with kubernetes, can support others)
    /// GRPC to worker implementation(will require registering with some discovery method), networks are heavyweight
    ///

    /// Notes
    ///
    /// Might require a separate event loop to dispatch events(generally clean anyway)
    ///
    println!("Hello, world!");
}
