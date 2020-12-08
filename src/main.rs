mod cli;

fn main() {
    // Take cli data

    /// Receive events
    /// How?
    /// Watch FS(easy over NFS, S3 and so on, lightweight, no services stood up)
    /// Wait over REST
    /// MQ(heavyweight??, requires client side implementation, could be a module)

    // Dispatch events(dispatcher)
    /// How might one dispatch events?
    /// FS(folders, worker prefix, worker then implements same watch interface)
    /// REST(post to workers that we know about, easy with kubernetes, can support others)
    println!("Hello, world!");
}
