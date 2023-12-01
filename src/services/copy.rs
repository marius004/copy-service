use std::{time::Duration, process};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::time;

use crate::models;

pub struct CopyService<'a> {
    config: &'a models::config::Config, 
    jobs: Vec<models::job::Job>,
}

impl<'a> CopyService<'a> {
    pub fn new(config: &'a models::config::Config) -> Self {
        CopyService {
            config,
            jobs: Vec::new(),
        }
    }

    pub async fn execute(&mut self) {
        let one_secs = Duration::from_secs(1);

        loop {
            println!("pid: {}", process::id());

            let mut job = models::job::Job::new(
                "/home/smarius/Documents/main.c",
                "/home/smarius/Documents/copy-service/copy.c",
            );

            let result = self.execute_job(&mut job).await;
            println!("{:?}", result);

            time::sleep(one_secs).await;
        }
    }

    async fn execute_job(&mut self, job: &mut models::job::Job) -> Result<String, Box<dyn std::error::Error>> {
        let mut source = BufReader::new(File::open(&job.source).await?);
        let mut destination = BufWriter::new(File::create(&job.destination).await?);

        let mut buffer: Vec<u8> = vec![0; 4096];
        while let Ok(bytes_read) = source.read(&mut buffer).await {
            println!("{}", bytes_read);

            if bytes_read == 0 {
                // TODO return stats
                break;
            }

            if let Err(err) = destination.write_all(&buffer[..bytes_read]).await {
                eprintln!("Error writing to destination: {}", err);
                // TODO handle error better
            }

            let status = job.status.lock().unwrap();
            if *status == models::job::JobStatus::Suspended {
                // TODO replace this with something else
                time::sleep(time::Duration::from_millis(100)).await;
            } else if *status == models::job::JobStatus::Canceled {
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
}