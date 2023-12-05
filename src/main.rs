use std::{
    fs::File,
    process,
};
use daemonize::Daemonize;

mod services;
mod models;
mod client;

use models::config::Config;
use client::client::Client;

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
        Ok(_) => Client::new(config).listen(),
        Err(err) => eprintln!("Error, {}", err),
    }
}
