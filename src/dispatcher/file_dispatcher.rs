use std::path::PathBuf;
use crate::dispatcher::DispatcherManager;
use crate::cli::Opts;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

pub struct FileJob {
    job_id: String,
    script_path: PathBuf,
    jobs_path: PathBuf,
}

impl FileJob {
    pub fn new(job_id: &String, cli_opts: &Opts) -> FileJob {
        FileJob {
            job_id: job_id.clone(),
            script_path: cli_opts.script_path.clone(),
            jobs_path: cli_opts.jobs_path.clone()
        }
    }
}

impl DispatcherManager for FileJob {
    fn dispatch(&self) {
        let directory = std::fs::read_dir(&self.jobs_path).expect("Failed to read job_path"); //TODO dont do this
        let workers = directory
            .filter(|directory| directory.is_ok())
            .map(|directory| directory.unwrap())
            .filter(|directory| directory.file_name().into_string().is_ok())
            .map(|directory| directory.file_name().into_string().unwrap())
            .filter(|directory| directory.contains("WORKER_"))
            .collect::<Vec<String>>();

        workers.iter().for_each(|worker| {
            let mut path = self.jobs_path.to_str().unwrap().to_string();
            if !path.ends_with("/") {
                path.push_str("/")
            }
            path.push_str(&worker);
            println!("Copying {:?} to {}", self.script_path, path);

            let mut options = CopyOptions::new();
            options.skip_exist = true;

            copy_items(&[&self.script_path], path, &options).unwrap();
            // std::fs::copy(&self.script_path, path).unwrap();
        })
        // find all workers in script path
        // do we have enough, if not batch them
        // write script file & job file to directory with key to determine we need to complete it
        // make sure permissions are correct
    }
}