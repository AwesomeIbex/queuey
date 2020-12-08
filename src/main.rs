mod cli;

fn main() {
    /// Receive events
    /// How?
    /// Watch FS(easy over NFS, S3 and so on, lightweight, no services stood up) - notify
    /// Wait over REST
    /// MQ(heavyweight??, requires client side implementation, could be a module) - zmq

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
