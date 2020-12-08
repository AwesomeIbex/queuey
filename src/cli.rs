use std::path::PathBuf;
use structopt::StructOpt;

/// Take cli data
///
/// Location or raw file
/// Pass a script(rust or bash script??, we could make it from a list of commands??)
/// into a folder with parameters as some format(YAML, JSON?)
/// whether it requires data callbacks!
/// file location
/// workers, how many? (Kubernetes could create them, should be quick to spawn)

#[derive(Debug, StructOpt)]
#[structopt(
name = "Worky",
about = "A simple, IO based worker queue",
version = "0.0.1",
author = "Donovan Dall - awesomealpineibex@gmail.com"
)]
pub struct Opts {
    /// Turn the app to debug mode (logs stuff)
    #[structopt(short, long)]
    pub debug: bool,

    // /// The amount of workers to deploy to
    // #[structopt(short = "w", long = "workers", default_value = 5)]
    // pub workers: i32,

    /// The location of the script
    #[structopt(long = "script", short = "s", parse(from_os_str))]
    pub script_path: PathBuf,

    /// The directory the worky jobs will be stored and read from
    #[structopt(long = "jobs_dir", short = "j", parse(from_os_str), default_value = "/tmp/worky")]
    pub jobs_path: PathBuf,
}

pub fn get_opts_args() -> Opts {
    Opts::from_args()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_opts_args() {
        
    }
}