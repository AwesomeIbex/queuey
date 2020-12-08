use std::fmt::Debug;

use crate::cli::Opts;
use crate::dispatcher::file_dispatcher::FileJob;

mod file_dispatcher;
// Dispatch events(dispatcher)
// How might one dispatch events?
// FS(folders, worker prefix, worker then implements same watch interface)
// REST(post to workers that we know about, easy with kubernetes, can support others)
// GRPC to worker implementation(will require registering with some discovery method), networks are heavyweight
//
pub enum DispatchType {
    FILE
}

trait DispatcherManager {
    fn dispatch(&self);
}

pub fn dispatch(job_id: &String, cli_opts: &Opts, dispatch_type: DispatchType) {
    // determine how to dispatch
    match dispatch_type {
        DispatchType::FILE => {
            let job = FileJob::new(&job_id, &cli_opts);
            job.dispatch()
        }
    }
}

pub fn dispatch_error<T: Debug>(error: T) {
    println!("watch error: {:?}", error);
}