use std::collections::HashSet;
use std::io::{Read, Seek, SeekFrom, 
    Write, BufWriter, BufReader};
use std::fs::{File, OpenOptions, metadata};
use std::path::Path;
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
        // create all destination directories 
        for dir_path in &job.destination_dirs {
            println!("Destination {}", dir_path);
            
            if let Err(err) = std::fs::create_dir_all(dir_path) {
                if err.kind() != std::io::ErrorKind::AlreadyExists {
                    eprintln!("Error creating directory {}: {}", dir_path, err);
                    return Err(err.into());
                }
            }
        }
        
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

    fn subjobs(job: Arc<Job>) -> Vec<Arc<Job>> {
        if CopyService::is_file(&job.source) {
            return vec![job];
        }

        let source_paths = CopyService::visit_directory(Path::new(&job.source));
        let mut destination_dirs: HashSet<String> = source_paths
            .clone()
            .into_iter()
            .filter_map(|path| {
              if CopyService::is_dir(&path) {
                Some(path.replace(&job.source, &job.destination))
              } else {
                None 
              } 
            })
            .collect();

        destination_dirs.insert(job.destination.clone());

        let subjobs: Vec<Arc<Job>> = source_paths
            .iter()
            .map(|path| {
                let destination = path.replace(&job.source, &job.destination);
                Arc::new(Job::new(
                    path.to_owned(), 
                    destination.clone(), 
                    Some(job.clone()), 
                    destination_dirs.clone()))
            })
            .collect();

        subjobs
    }

    fn visit_directory(directory: &Path) -> Vec<String> {
        std::fs::read_dir(directory)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        match path.is_file() {
                            true => Some(vec![path.to_str().unwrap().to_string()]),
                            false if path.is_dir() => Some(CopyService::visit_directory(&path)),
                            _ => None,
                        }
                    }
                    Err(err) => {
                        eprintln!("Unexpected error during directory walk, {}", err);
                        None
                    }
                }
            })
            .flatten()
            .collect()
    }
        
    fn is_file(path: &str) -> bool {
        metadata(path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    fn is_dir(path: &str) -> bool {
        metadata(path)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    fn update_job_status(job: Arc<Job>, new_status: JobStatus) {
        let mut status = job.status.write().unwrap();
        *status = new_status;
    }

    fn increase_job_writes(job: Arc<Job>) {
        let mut writes = job.writes.write().unwrap(); 
        *writes += 1;
    }

    pub fn add_job(&mut self, job: Job) {
        for subjob in CopyService::subjobs(Arc::new(job)) {
            self.jobs.push(subjob);   
        }
    }
}
