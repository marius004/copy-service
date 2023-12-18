use std::{fmt::Debug, sync::{Arc, RwLock}, fs};
use serde::{Deserialize, Serialize};

use crate::models::{job::{Job, JobStatus}, config::Config};

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
pub struct ResumeResponse {
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
    pub percentage: f64, 
}

impl JobResponse {
    pub fn from_job(job: &Arc<Job>, config: Arc<RwLock<Config>>) -> Self {
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
            percentage: JobResponse::percentage(writes, job.source.to_owned(), config),
        }
    }

    fn percentage(writes: u64, source_path: String, config: Arc<RwLock<Config>>) -> f64 {
        if let Ok(metadata) = fs::metadata(source_path) {
            let source_bytes = metadata.len();
            let buffer_size = config.read().unwrap().buffer_size as f64;

            let percentage = (writes as f64 * buffer_size) / source_bytes as f64;

            if percentage > 1.0 {
                1.0
            } else {
                percentage
            }
        } else {
            0.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressResponse {
    pub job: JobResponse,
}