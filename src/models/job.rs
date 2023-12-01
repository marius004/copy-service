use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Job {
    pub source: &'static str,
    pub destination: &'static str,
    pub status: Arc<Mutex<JobStatus>>,
    pub writes: u64, // nr. of successful writes to the destination file
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
            writes: 0,
        }
    }
}
