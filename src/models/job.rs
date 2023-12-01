use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Job {
    pub source: &'static str,
    pub destination: &'static str,
    pub status: Arc<Mutex<JobStatus>>,
}

#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Created,
    Running,
    Suspended,
    Completed,
    Canceled,
}

impl Job {
    pub fn new(source: &'static str, destination: &'static str) -> Self {
        Job {
            source: source, 
            destination: destination,
            status: Arc::new(Mutex::new(JobStatus::Created)),
        }
    }
}
