use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub source: String,
    pub destination: String,
    pub status: Arc<RwLock<JobStatus>>,
    pub writes: Arc<RwLock<u64>>, // nr. of successful writes to the destination file
}

#[derive(Debug, PartialEq, Clone)]
pub enum JobStatus {
    Created,
    Running,
    Suspended,
    Resumed, 
    Completed,
    Canceled,
    Failed(String),
}

impl Job {
    pub fn new(source: String, destination: String) -> Self {
        Job {
            id: Uuid::new_v4(),
            source: source, 
            destination: destination,
            status: Arc::new(RwLock::new(JobStatus::Created)),
            writes: Arc::new(RwLock::new(0u64)),
        }
    }
}
