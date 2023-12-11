use std::time::Duration;
use std::sync::Arc;
use super::job::Job;

#[derive(Debug)]
pub struct CopyStats {
    pub job: Arc<Job>,
    pub time: Duration,
}

impl CopyStats {
    pub fn new(job: Arc<Job>, time: Duration) -> Self {
        CopyStats {
            job, 
            time,
        }
    }
}