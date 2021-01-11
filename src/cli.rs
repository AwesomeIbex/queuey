use std::path::PathBuf;
use structopt::StructOpt;
use std::str::FromStr;
use std::string::ParseError;
use anyhow::{Error, anyhow};

#[derive(Debug)]
pub enum Platform {
    Local, Kubernetes
}
impl FromStr for Platform {
    type Err = Error;
    fn from_str(day: &str) -> Result<Self, Error> {
        match day {
            "kubernetes" | "k8s" => Ok(Platform::Kubernetes),
            "local" => Ok(Platform::Local),
            _ => Err(anyhow!("Could not parse a platform")),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Queuey",
about = "A simple, IO based worker queue manager",
version = "0.0.1",
author = "Donovan Dall - awesomealpineibex@gmail.com"
)]
pub struct Opts {
    // Determine the platform for execution
    #[structopt(short = "p", long = "platform", default_value = "local")]
    pub platform: Platform,

    /// The amount of workers to deploy to
    #[structopt(short = "w", long = "workers", default_value = "15")]
    pub workers: i32,

    /// The location of the script
    #[structopt(long = "script", short = "s", parse(from_os_str))]
    pub script_path: PathBuf,

    /// The directory the queuey jobs will be stored and read from
    #[structopt(long = "jobs_dir", short = "j", parse(from_os_str), default_value = "/tmp/queuey")]
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