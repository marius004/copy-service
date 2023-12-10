use std::{fmt::Debug, sync::Arc};
use serde::{Deserialize, Serialize};

use crate::models::job::Job;

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
        let status = *job.status.read().unwrap();
        let writes = *job.writes.read().unwrap();
        
        JobResponse { 
            id: job.id.to_string(),
            source: job.source.clone(), 
            destination: job.destination.clone(),
            status: format!("{:#?}", status),
            writes: writes,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressResponse {
    pub job: JobResponse,
}