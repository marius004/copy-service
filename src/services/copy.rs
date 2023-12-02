use std::io::{Read, Seek, SeekFrom, 
    Write, BufWriter, BufReader};
use std::fs::{File, OpenOptions};
use std_semaphore::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use std::thread;

use crate::models::job::{Job, JobStatus};
use crate::models::copy::CopyStats;
use crate::models::config::Config;

pub struct CopyService {
    config: Arc<Config>,
    jobs: Vec<Arc<Job>>,
    semaphore: Arc<Semaphore>,
}

impl CopyService {
    pub fn new(config: Config) -> Self {
        CopyService {
            config: Arc::new(config.clone()),
            jobs: Vec::new(),
            semaphore: Arc::new(Semaphore::new(config.max_threads as isize)),
        }
    }

    pub fn execute(&mut self) { 
        let handles: Vec<_> = 
            self.jobs
                .to_owned()
                .into_iter()
                .map(|job| {
                    let (config_clone, job_clone, semaphore_clone) = (
                        Arc::clone(&self.config),
                        Arc::clone(&job),
                        Arc::clone(&self.semaphore),
                    );

                    self.semaphore.acquire();
                    thread::spawn(move || {
                        let result = CopyService::execute_job(&config_clone, &job_clone);
                        semaphore_clone.release();
                        result
                    })
                })
                .collect();

        handles
            .into_iter()
            .for_each(|handle| {
                match handle.join() {
                    Ok(result) => println!("{:#?}", result),
                    Err(error) => eprintln!("{:?}", error)
                }
            });
    }

    fn execute_job(config: &Config, job: &Arc<Job>) -> Result<CopyStats> {
        let mut source = CopyService::source_reader(config, job.clone())?;
        let mut destination = CopyService::destination_writer(job.clone())?;

        let mut buffer: Vec<u8> = vec![0; config.buffer_size];
        while let Ok(bytes_read) = source.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            match destination.write_all(&buffer[..bytes_read]) {
                Ok(_) => CopyService::increase_job_writes(job.clone()),
                Err(err) => {
                    let error_message = format!("Error writing to destination file, {}", err.to_string());
                    CopyService::update_job_status(job.clone(), JobStatus::Failed(error_message));

                    return Ok(CopyStats::new(job.clone(), Duration::from_secs(0)));
                },
            }

            if *job.status.read().unwrap() == JobStatus::Suspended ||
               *job.status.read().unwrap() == JobStatus::Canceled {
                destination.flush()?;
                return Ok(CopyStats::new(job.clone(), Duration::from_secs(0)));
            }
        }

        destination.flush()?;
        Ok(CopyStats::new(job.clone(), Duration::from_secs(0)))
    }

    fn source_reader(config: &Config, job: Arc<Job>) -> Result<BufReader<File>> {
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

    fn destination_writer(job: Arc<Job>) -> Result<BufWriter<File>> {
        let destination = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&job.destination)?;

        Ok(BufWriter::new(destination))
    }

    fn update_job_status(job: Arc<Job>, new_status: JobStatus) {
        let mut status = job.status.write().unwrap();
        *status = new_status;
    }

    fn increase_job_writes(job: Arc<Job>) {
        let mut writes = job.writes.write().unwrap(); 
        *writes += 1;
    }

    // TODO remove this when other way of communication will be implemented
    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(Arc::new(job));
    }
}
