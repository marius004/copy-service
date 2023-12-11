use std::sync::{Arc, RwLock};
use std::str::FromStr;
use uuid::Uuid;

use crate::models::job::{Job, JobStatus};

pub struct StorageService {
    jobs: Arc<RwLock<Vec<Arc<Job>>>>,
} 

impl StorageService {
    pub fn new() -> Self {
        StorageService { 
            jobs: Arc::new(RwLock::new(Vec::new()))
        }
    }

    pub fn suspend_job(&mut self, job_id: String) -> bool {
        Uuid::from_str(&job_id)
            .ok()
            .and_then(|uuid| self.jobs.write().unwrap().iter_mut().find(|job| job.id == uuid).cloned())
            .map(|job| {
                let mut status = job.status.write().unwrap();
                *status = JobStatus::Suspended;
            })
            .is_some()
    }

    pub fn cancel_job(&mut self, job_id: String) -> bool {
        Uuid::from_str(&job_id)
            .ok()
            .and_then(|uuid| self.jobs.write().unwrap().iter_mut().find(|job| job.id == uuid).cloned())
            .map(|job| {
                let mut status = job.status.write().unwrap();
                *status = JobStatus::Canceled;
            })
            .is_some()
    }

    pub fn job(&self, job_id: String) -> Option<Arc<Job>> {
        Uuid::from_str(&job_id)
            .map_err(|e| eprintln!("Error parsing job ID: {}", e))
            .ok()
            .and_then(|uuid| self.jobs.read().ok().map(|job| (uuid, job)))
            .and_then(|(uuid, jobs)| jobs.iter().find(|job| job.id == uuid).cloned())
    }

    pub fn jobs(&self) -> Arc<RwLock<Vec<Arc<Job>>>> {
        self.jobs.clone()
    }

    pub fn add_job(&mut self, job: Job) -> Arc<Job> {  
        let mut jobs = self.jobs.write().unwrap();
        let job_arc = Arc::new(job);
        jobs.push(Arc::clone(&job_arc));
        job_arc
    }

    pub fn increment_job_writes(job: Arc<Job>) {
        let mut writes = job.writes.write().unwrap(); 
        *writes += 1;
    }

    pub fn update_job_status(job: Arc<Job>, new_status: JobStatus) {
        let mut status = job.status.write().unwrap();
        *status = new_status;
    }
}