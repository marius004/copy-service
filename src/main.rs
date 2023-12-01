use std::fs;
use toml;
use std::process;
use daemonize::Daemonize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    pid_file: String,
    working_directory: String,
    stdout_file: String,
    stderr_file: String,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        toml::from_str(&config_str).map_err(Into::into)
    }
}

fn main() {
    let config = match Config::from_file("./Config.toml") {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error encountered when reading/parsing config file: {}", err);
            process::exit(1);
        }
    };

    let stdout = fs::File::create(&config.stdout_file).unwrap();
    let stderr = fs::File::create(&config.stderr_file).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(config.pid_file)
        .working_directory(config.working_directory)
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(v) => {
            println!("{:?}", v);
            println!("Success, daemonized");
            run();
        }
        Err(err) => eprintln!("Error, {}", err),
    }
}

fn run() {
    let one_secs = std::time::Duration::from_secs(1);

    loop {
        println!("pid: {}", std::process::id());
        std::thread::sleep(one_secs);
    }
}
