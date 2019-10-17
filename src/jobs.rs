use bloom_error::BloomError;
use bloom_kernel::config;
use failure::Fail;
use std::collections::HashMap;
use std::error::Error;

type Job = fn(config::Config) -> Result<(), BloomError>;

fn get_jobs() -> HashMap<String, Job> {
    let map = HashMap::new();
    return map;
}

pub fn list_jobs() -> Result<(), Box<dyn Error>> {
    let jobs = get_jobs();
    jobs.keys().for_each(|j| println!("{}", j));
    return Ok(());
}

pub fn run_job(cfg: config::Config, job: &str) -> Result<(), Box<dyn Error>> {
    let jobs = get_jobs();

    match jobs.get(job) {
        Some(job) => {
            match job(cfg) {
                Err(err) => return Err(Box::new(err.compat())),
                Ok(_) => return Ok(()),
            };
        }
        None => {
            return Err(Box::new(
                BloomError::Validation(format!("job: {} not found", job)).compat(),
            ))
        }
    };
}
