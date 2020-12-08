use std::fmt::Debug;

// Dispatch events(dispatcher)
// How might one dispatch events?
// FS(folders, worker prefix, worker then implements same watch interface)
// REST(post to workers that we know about, easy with kubernetes, can support others)
// GRPC to worker implementation(will require registering with some discovery method), networks are heavyweight
//

trait DispatcherWorker<T> {
    fn dispatch(event: T);
}

pub fn dispatch<T: Debug>(event: T) {
    // determine how to dispatch
    println!("{:?}", event);
}
pub fn dispatch_error<T: Debug>(error: T) {
    println!("watch error: {:?}", error);
}