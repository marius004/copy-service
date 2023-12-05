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
    pub base: JobRequest,

    pub source_path: String,
    pub destination_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspendJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    #[serde(flatten)]
    pub base: JobRequest,
}

pub enum AnyRequest {
    Copy(CopyJobRequest),
    Suspend(SuspendJobRequest),
    Cancel(CancelJobRequest),
    Progress(ProgressJobRequest),
    List(ListRequest),
}

pub fn parse_request(json_str: &str) -> Result<AnyRequest> {
    let value: Value = serde_json::from_str(json_str)?;

    let req_type: JobRequestType = match serde_json::from_value(value.get("request_type").unwrap().clone()) {
        Ok(result) => result,
        Err(_) => return Err(anyhow!("Failed to deserialize request_type")),
    };

    let result = match req_type {
        JobRequestType::Copy => {
            let copy_request: CopyJobRequest = serde_json::from_str(json_str)?;
            AnyRequest::Copy(copy_request)
        }
        JobRequestType::Suspend => {
            let suspend_request: SuspendJobRequest = serde_json::from_str(json_str)?;
            AnyRequest::Suspend(suspend_request)
        }
        JobRequestType::Cancel => {
            let cancel_request: CancelJobRequest = serde_json::from_str(json_str)?;
            AnyRequest::Cancel(cancel_request)
        }
        JobRequestType::Progress => {
            let progress_request: ProgressJobRequest = serde_json::from_str(json_str)?;
            AnyRequest::Progress(progress_request)
        }
        JobRequestType::List => {
            let list_request: ListRequest = serde_json::from_str(json_str)?;
            AnyRequest::List(list_request)
        }
        _ => return Err(anyhow!("Unknown request type")),
    };

    Ok(result)
}
