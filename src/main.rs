use std::{
    fs::File,
    process, sync::{mpsc::channel, Arc, RwLock, Mutex}, thread,
};
use daemonize::Daemonize;

mod services;
mod models;
mod client;

use models::{config::Config, job::Job};
use client::client::Client;
use services::{storage::StorageService, copy::CopyService};

fn run(config: Config) {
    let (sender, receiver) = channel::<Job>();
    
    let storage_service = Arc::new(RwLock::new(StorageService::new()));
    let copy_service =  Arc::new(RwLock::new(CopyService::new(Arc::new(config.clone()), Mutex::new(receiver), storage_service.clone())));
    let client_service = Arc::new(Mutex::new(Client::new(storage_service.clone(), sender,  Arc::new(RwLock::new(config.clone())))));

    let client_handle = thread::spawn(move || {
        client_service.lock().unwrap().listen();
    });

    copy_service.write().unwrap().execute();
    client_handle.join().unwrap();
}

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
        Ok(_) => run(config),
        Err(err) => eprintln!("Error, {}", err),
    }
}
