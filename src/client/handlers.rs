use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;
use anyhow::Result;

use crate::client::requests::*;
use crate::client::responses::*;
use crate::models::job::Job;
use crate::services::storage::StorageService;

pub fn handle_copy(request: CopyJobRequest, sender: Sender<Job>)
    -> Result<String> {
    let job = Job::new(
        request.source_path,
        request.destination_path,
        None,
        HashSet::new(),
    );

    match sender.send(job.clone()) {
        Ok(_) => 
            Ok(serde_json::to_string(&CopyResponse{ job_id: job.id.to_string() })?),
        Err(err) => 
            Ok(serde_json::to_string(&ErrorMessageResponse{ message: format!("Could not copy, {}", err) })?),
    } 
}

pub fn handle_suspend(request: SuspendJobRequest, storage_service: Arc<RwLock<StorageService>>) 
    -> Result<String> {
    
    match storage_service.write().unwrap().suspend_job(request.job_id.clone()) {
        true => 
            Ok(serde_json::to_string(&SuspendResponse { message: format!("Job {} suspended successfully", request.job_id) })?),
        false => 
            Ok(serde_json::to_string(&ErrorMessageResponse{ message: format!("Could not suspend job {}", request.job_id) })?),
    }
}

pub fn handle_cancel(request: CancelJobRequest, storage_service: Arc<RwLock<StorageService>>) -> Result<String> {
    match storage_service.write().unwrap().cancel_job(request.job_id.clone()) {
        true => 
            Ok(serde_json::to_string(&CancelResponse { message: format!("Job {} cancelled successfully", request.job_id) })?),
        false => 
            Ok(serde_json::to_string(&ErrorMessageResponse { message: format!("Could not cancel job {}", request.job_id) })?),
    }
}

pub fn handle_progress(request: ProgressJobRequest, storage_service: Arc<RwLock<StorageService>>) -> Result<String> {
    match storage_service.read().unwrap().job(request.job_id.clone()) {
        Some(stats) => 
            Ok(serde_json::to_string(&JobResponse::from_job(&stats))?),
        None => 
           Ok(serde_json::to_string(&ErrorMessageResponse { message: format!("Could not find job {}", request.job_id) })?),
    }
}

pub fn handle_list(storage_service: Arc<RwLock<StorageService>>) -> Result<String> {
    let active_jobs: Vec<_> = 
        storage_service
            .read()
            .unwrap()
            .jobs()
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .collect();

    let response: Vec<_> = 
        active_jobs
            .iter()
            .map(|job| JobResponse::from_job(job))
            .collect();

    Ok(serde_json::to_string(&response)?)
}

pub fn handle_error(error: anyhow::Error) -> Result<String> {
    Ok(serde_json::to_string(&ErrorMessageResponse { message:error.to_string() })?)
}