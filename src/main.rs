use std::{fs::File, process};
use daemonize::Daemonize;

mod services;
mod models;

use models::{config::Config, job::Job};
use services::copy::CopyService;

fn main() {
    let config = match Config::from_file("./Config.toml") {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error encountered when reading/parsing config file: {}", err);
            process::exit(1);
        }
    };

    let stdout = File::create(&config.stdout_file).unwrap();
    let stderr = File::create(&config.stderr_file).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(&config.pid_file)
        .working_directory(&config.working_directory)
        .stdout(stdout)
        .stderr(stderr);
    
    match daemonize.start() {
        Ok(_) => {
            let mut cs = CopyService::new(&config);
            cs.add_job(Job::new("/home/smarius/Documents/main.c",
            "/home/smarius/Documents/copy-service/copy.c"));

            cs.add_job(Job::new("/home/smarius/Documents/main.c",
            "/home/smarius/Documents/copy-service/copy2.c"));

            cs.add_job(Job::new("/home/smarius/Documents/main.c",
            "/home/smarius/Documents/copy-service/copy3.c"));

            cs.add_job(Job::new("/home/smarius/Documents/main.c",
            "/home/smarius/Documents/copy-service/copy4.c"));
        
            cs.execute()
        },
        Err(err) => {eprintln!("Error, {}", err)},
    }
}