use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{anyhow, Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobRequestType {
    Copy,
    Cancel, 
    Suspend,
    Progress,
    List, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequest {
    request_type: JobRequestType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyJobRequest {
    #[serde(flatten)]
    base: JobRequest,

    source_path: String,
    destination_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspendJobRequest {
    #[serde(flatten)]
    base: JobRequest,

    job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelJobRequest {
    #[serde(flatten)]
    base: JobRequest,

    job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressJobRequest {
    #[serde(flatten)]
    base: JobRequest,

    job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    #[serde(flatten)]
    base: JobRequest,
}

pub trait RequestTrait : Debug {}

impl RequestTrait for CopyJobRequest {}
impl RequestTrait for SuspendJobRequest {}
impl RequestTrait for CancelJobRequest {}
impl RequestTrait for ProgressJobRequest {}
impl RequestTrait for ListRequest {}

pub fn parse_request(json_str: &str) -> Result<Box<dyn RequestTrait>> {
    let value: Value = serde_json::from_str(json_str)?;

    let req_type: JobRequestType = match serde_json::from_value(value.get("request_type").unwrap().clone()) {
        Ok(result) => result,
        Err(_) => return Err(anyhow!("Failed to deserialize request_type")),
    };

    let result: Box<dyn RequestTrait> = match req_type {
        JobRequestType::Copy => {
            let copy_request: CopyJobRequest = serde_json::from_str(json_str)?;
            Box::new(copy_request)
        }
        JobRequestType::Suspend => {
            let suspend_request: SuspendJobRequest = serde_json::from_str(json_str)?;
            Box::new(suspend_request)
        }
        JobRequestType::Cancel => {
            let cancel_request: CancelJobRequest = serde_json::from_str(json_str)?;
            Box::new(cancel_request)
        }
        JobRequestType::Progress => {
            let progress_request: ProgressJobRequest = serde_json::from_str(json_str)?;
            Box::new(progress_request)
        }
        JobRequestType::List => {
            let list_request: ListRequest = serde_json::from_str(json_str)?;
            Box::new(list_request)
        }
        _ => return Err(anyhow!("Unknown request type")),
    };

    Ok(result)
}
