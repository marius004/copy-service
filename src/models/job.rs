use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Job {
    pub source: &'static str,
    pub destination: &'static str,
    pub status: Arc<RwLock<JobStatus>>,
    pub writes: Arc<RwLock<u64>>, // nr. of successful writes to the destination file
}

#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Created,
    Running,
    Suspended,
    Completed,
    Canceled,
    Failed,
}

impl Job {
    pub fn new(source: &'static str, destination: &'static str) -> Self {
        Job {
            source: source, 
            destination: destination,
            status: Arc::new(RwLock::new(JobStatus::Created)),
            writes: Arc::new(RwLock::new(0u64)),
        }
    }
}
