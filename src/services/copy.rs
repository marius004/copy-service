use std::io::{Read, Seek, SeekFrom, Write, BufWriter, BufReader};
use std::fs::{File, OpenOptions};
use std_semaphore::Semaphore;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use anyhow::Result;
use std::thread;

use crate::models::job::{Job, JobStatus};
use crate::models::config::Config;

pub struct CopyService {
    config: Arc<Config>,
    jobs: Vec<Arc<RwLock<Job>>>,
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

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(Arc::new(RwLock::new(job)));
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
                if let Some(error) = handle.join().err() {
                    eprintln!("{:?}", error);
                }
            });
    }

    fn execute_job(config: &Config, jlock: &Arc<RwLock<Job>>) -> Result<String> {
        let rjob = jlock.read().unwrap(); 

        let mut source = CopyService::source_reader(config, &*rjob)?;
        let mut destination = CopyService::destination_writer(&*rjob)?;

        let mut buffer: Vec<u8> = vec![0; config.buffer_size];
        while let Ok(bytes_read) = source.read(&mut buffer) {
            println!("Bytes read {}", bytes_read);
            if bytes_read == 0 {
                // TODO return stats
                break;
            }

            if let Err(err) = destination.write_all(&buffer[..bytes_read]) {
                eprintln!("Error writing to destination: {}", err);
                // TODO handle error better
            }

            if rjob.status == JobStatus::Suspended {
                // TODO replace this with something else
                thread::sleep(Duration::from_millis(100));
            } else if rjob.status == JobStatus::Canceled {
                // TODO replace this with something else
                // should the destination file be discarded?
                return Ok(String::from(""));
            }
        }

        // todo: update the status of the job
        // todo: don't forget to flush the buffers also when returning
        destination.flush()?;
        Ok(String::from(""))
    }

    fn source_reader(config: &Config, job: &Job) -> Result<BufReader<File>> {
        match job.writes {
            writes if writes > 0 => {
                let offset = config.buffer_size as u64 * writes;
                let mut source = OpenOptions::new().read(true).open(&job.source)?;

                source.seek(SeekFrom::Start(offset))?;
                Ok(BufReader::new(source))
            }
            _ => Ok(BufReader::new(File::open(&job.source)?)),
        }
    }

    fn destination_writer(job: &Job) -> Result<BufWriter<File>> {
        let destination = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&job.destination)?;

        Ok(BufWriter::new(destination))
    }
}
