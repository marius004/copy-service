use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write, BufWriter, BufReader};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use anyhow::Result;
use std::process;
use std::thread;

use crate::models::job::{Job, JobStatus};
use crate::models::config::Config;

pub struct CopyService<'a> {
    config: &'a Config,
    jobs: Vec<Arc<RwLock<Job>>>,
}

impl<'a> CopyService<'a> {
    pub fn new(config: &'a Config) -> Self {
        CopyService {
            config,
            jobs: Vec::new(),
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(Arc::new(RwLock::new(job)));
    }

    pub fn execute(&mut self) {
        println!("{}", process::id());
        let mut handles = vec![];

        for job in &self.jobs {
            println!("{:?}", job);

            let config = self.config.clone();
            let job_clone = Arc::clone(job);

            let handle = thread::spawn(move || {
                let result = CopyService::execute_job(&config, &job_clone);
                result
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(err) = handle.join() {
                eprintln!("Error in spawned thread: {:?}", err);
                println!("Error in spawned thread: {:?}", err);
            } else {
                println!("Thread completed successfully");
            }
        }
    }

    fn execute_job(config: &Config, jlock: &Arc<RwLock<Job>>) -> Result<String> {
        let rjob = jlock.read().unwrap(); // Obtain a lock to the job

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
