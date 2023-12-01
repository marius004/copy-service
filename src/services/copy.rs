use std::{time::Duration, thread, process};
use crate::models;

pub struct CopyService<'a> {
    config: &'a models::config::Config, 
    jobs: Vec<models::job::Job>,
}

impl<'a> CopyService<'a> {
    pub fn new(config: &'a models::config::Config) -> Self {
        CopyService {
            config: config, 
            jobs: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        let one_secs = Duration::from_secs(1);

        loop {
            println!("pid: {}", process::id());
            thread::sleep(one_secs);
        }
    }
}