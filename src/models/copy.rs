use std::time::Duration;
use std::sync::Arc;
use super::job::Job;


#[derive(Debug)]
pub struct CopyStats {
    job: Arc<Job>,
    time: Duration,
}

impl CopyStats {
    pub fn new(job: Arc<Job>, time: Duration) -> Self {
        CopyStats {
            job, 
            time,
        }
    }
}