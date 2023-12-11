use std::io::{Read, Seek, SeekFrom, Write, BufWriter, BufReader};
use std::fs::{File, OpenOptions};
use std::sync::{Arc, RwLock, Mutex};
use std::sync::mpsc::Receiver;
use anyhow::{Result, anyhow};
use threadpool::ThreadPool;
use std::time::Duration;
use std::thread;

use crate::services::storage::StorageService;
use crate::models::job::{Job, JobStatus};
use crate::models::config::Config;
use crate::services::validate::validate;

pub struct CopyService {
    config: Arc<Config>,
    storage: Arc<RwLock<StorageService>>,
    receiver: Mutex<Receiver<Job>>,
    workers: ThreadPool,
}

impl CopyService {
    pub fn new(config: Arc<Config>, receiver: Mutex<Receiver<Job>>, storage: Arc<RwLock<StorageService>>) -> Self {
        let workers = ThreadPool::new(config.max_threads as usize);

        CopyService {
            config: config,
            receiver: receiver,
            storage: storage,
            workers: workers, 
        }
    }

    pub fn execute(&mut self) {
        loop {
            if let Ok(data) = self.receiver.lock().unwrap().try_recv() {
                let job = self.storage.write().unwrap().add_job(data);
                let config_clone = Arc::clone(&self.config);

                self.workers.execute(move || {
                    if let Err(err) = CopyService::execute_job(&config_clone, job.clone()) {
                        StorageService::update_job_status(job, JobStatus::Failed(err.to_string()));
                    }
                });
            }
        }
    }
    
    fn execute_job(config: &Arc<Config>, job: Arc<Job>) -> Result<Arc<Job>> {
        StorageService::update_job_status(job.clone(), JobStatus::Running);
        if let(false, message) = validate(job.clone()) {
            return Err(anyhow!(message));
        }

        let mut source = CopyService::source_reader(config, job.clone())?;
        let mut destination = CopyService::destination_writer(&job.clone())?;
        
        let mut buffer: Vec<u8> = vec![0; config.buffer_size.clone()];
        while let Ok(bytes_read) = source.read(&mut buffer) {
            if bytes_read == 0 {
                StorageService::update_job_status(job.clone(), JobStatus::Completed);
                break;
            }

            match destination.write_all(&buffer[..bytes_read]) {
                Ok(_) => StorageService::increment_job_writes(job.clone()),
                Err(_) => {
                    StorageService::increment_job_writes(job.clone());
                    return Ok(job.clone());
                },
            }

            if config.testing {
                thread::sleep(Duration::from_secs_f32(config.delay));
            }

            if *job.status.read().unwrap() == JobStatus::Suspended ||
               *job.status.read().unwrap() == JobStatus::Canceled {
                destination.flush()?;
                return Ok(job.clone());
            }
        }

        destination.flush()?;
        Ok(job.clone())
    }

    fn source_reader(config: &Arc<Config>, job: Arc<Job>) -> Result<BufReader<File>> {
        match *job.writes.read().unwrap() {
            writes if writes > 0 => {
                let offset = config.buffer_size as u64 * writes;
                let mut source = OpenOptions::new().read(true).open(&job.source)?;

                source.seek(SeekFrom::Start(offset))?;
                Ok(BufReader::new(source))
            }
            _ => Ok(BufReader::new(File::open(&job.source)?)),
        }
    }

    fn destination_writer(job: &Arc<Job>) -> Result<BufWriter<File>> {
        let destination = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&job.destination)?;

        Ok(BufWriter::new(destination))
    }
}