use std::path::PathBuf;
use crate::dispatcher::DispatcherManager;
use crate::cli::Opts;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const WORKER_PREFIX: &str = "WORKER_";

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
    fn build_path(&self, worker: &String) -> String {
        let mut path = self.jobs_path.to_str().unwrap().to_string();
        if !path.ends_with("/") {
            path.push_str("/")
        }
        path.push_str(&worker);
        path.push_str("/");
        path.push_str(&self.job_id);
        path
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
            .filter(|directory| directory.starts_with(&WORKER_PREFIX))
            .collect::<Vec<String>>();

        // TODO do we have enough, if not batch them/create them

        // TODO move me
        let mut options = CopyOptions::new();
        options.skip_exist = true;

        workers.par_iter().for_each(|worker| {
            let mut path = self.build_path(&worker);
            // Permissions are messed up here
            std::fs::create_dir_all(&path).unwrap();
            println!("Copying {:?} to {}", self.script_path, path);

            // write script file & job file to directory with key to determine we need to complete it
            copy_items(&[&self.script_path], path, &options).unwrap();
            // make sure permissions are correct
        })
    }
}