use std::{fmt::Debug, sync::Arc};
use serde::{Deserialize, Serialize};

use crate::models::job::{Job, JobStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyResponse {
    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspendResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobResponse {
    pub id: String,
    pub source: String,
    pub destination: String,

    pub status: String,
    pub writes: u64, 
}

impl JobResponse {
    pub fn from_job(job: &Arc<Job>) -> Self {
        let job_clone = job.clone();
        let writes = *job.writes.read().unwrap();

        let status = {
            let status_guard = job_clone.status.read().unwrap();
            match &*status_guard {
                JobStatus::Failed(message) => format!("failed: {}", message),
                other => format!("{:?}", other),
            }
        };
        
        JobResponse { 
            id: job.id.to_string(),
            source: job.source.clone(), 
            destination: job.destination.clone(),
            status: status,
            writes: writes,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressResponse {
    pub job: JobResponse,
}