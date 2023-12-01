use std::io::SeekFrom;
use std::{time::Duration, process};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use std::error::Error;
use tokio::time;

use crate::models::config::Config;
use crate::models::job::Job;
use crate::models::job::JobStatus;
use crate::models::types::Action;

pub struct CopyService<'a> {
    config: &'a Config, 
    jobs: Vec<Job>,
}

impl<'a> CopyService<'a> {
    pub fn new(config: &'a Config) -> Self {
        CopyService {
            config,
            jobs: Vec::new(),
        }
    }

    pub async fn execute(&mut self) {
        let one_secs = Duration::from_secs(1);
    
        println!("pid: {}", process::id());

        let mut job = Job::new(
            "/home/smarius/Documents/main.c",
            "/home/smarius/Documents/copy-service/copy.c",
        );

        let result = self.execute_job(&mut job).await;
        println!("{:?}", result);

        time::sleep(one_secs).await;
    }

    async fn execute_job(&mut self, job: &mut Job) -> Action<String> {
        let mut source = self.source_reader(job).await?;
        let mut destination = self.destination_writer(job).await?;

        let mut buffer: Vec<u8> = vec![0; self.config.buffer_size];
        while let Ok(bytes_read) = source.read(&mut buffer).await {
            if bytes_read == 0 {
                // TODO return stats
                break;
            }

            if let Err(err) = destination.write_all(&buffer[..bytes_read]).await {
                eprintln!("Error writing to destination: {}", err);
                // TODO handle error better
            }

            job.writes += 1;

            let status = job.status.lock().unwrap();
            if *status == JobStatus::Suspended {
                // TODO replace this with something else
                time::sleep(time::Duration::from_millis(100)).await;
            } else if *status == JobStatus::Canceled {
                // TODO replace this with something else
                // should the destination file be discarded?
                return Ok(String::from(""));
            }
        }

        // todo: update the status of the job
        // todo: don't forget to flush the buffers also when returning
        source.flush().await?;
        destination.flush().await?;

        Ok(String::from(""))
    }

    async fn source_reader(&self, job: &Job) -> Action<BufReader<File>> {
        match job.writes {
            writes if writes > 0 => {
                let offset = self.config.buffer_size as u64 * writes;
                let mut source =
                    OpenOptions::new()
                        .read(true)
                        .open(&job.source)
                        .await?;
                
                source.seek(SeekFrom::Start(offset)).await?;
                Ok(BufReader::new(source))
            }, 
            _ => Ok(BufReader::new(File::open(&job.source).await?))
        }
    }

    async fn destination_writer(&self, job: &Job) -> Action<BufWriter<File>> {
        let destination =
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&job.destination)
                .await?;

        Ok(BufWriter::new(destination))
    }
}