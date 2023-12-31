use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobRequestType {
    Copy,
    Cancel, 
    Suspend,
    Progress,
    List, 
    Resume,
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

    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressJobRequest {
    #[serde(flatten)]
    pub base: JobRequest,

    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListJobsRequest {
    #[serde(flatten)]
    pub base: JobRequest,
}

#[derive(Debug)]
pub enum AnyRequest {
    Copy(CopyJobRequest),
    Suspend(SuspendJobRequest),
    Cancel(CancelJobRequest),
    Progress(ProgressJobRequest),
    List(ListJobsRequest),
    Resume(ResumeJobRequest),
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
            let list_request: ListJobsRequest = serde_json::from_str(json_str)?;
            AnyRequest::List(list_request)
        }, 
        JobRequestType::Resume => {
            let resume_request: ResumeJobRequest = serde_json::from_str(json_str)?;
            AnyRequest::Resume(resume_request)
        }
    };

    Ok(result)
}
