use std::{sync::{Arc, RwLock}, collections::HashSet};

#[derive(Debug)]
pub struct Job {
    pub source: String,
    pub destination: String,
    pub status: Arc<RwLock<JobStatus>>,
    pub writes: Arc<RwLock<u64>>, // nr. of successful writes to the destination file
    
    pub parent: Option<Arc<Job>>,
    pub destination_dirs: HashSet<String>,
}

#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Created,
    Running,
    Suspended,
    Completed,
    Canceled,
    Failed(String),
}

impl Job {
    pub fn new(source: String, destination: String, 
        parent: Option<Arc<Job>>, 
        destination_dirs: HashSet<String>) -> Self {
        Job {
            source: source, 
            destination: destination,
            status: Arc::new(RwLock::new(JobStatus::Created)),
            writes: Arc::new(RwLock::new(0u64)),
            
            parent: parent, 
            destination_dirs: destination_dirs
        }
    }
}
