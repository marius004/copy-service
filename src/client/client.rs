use std::net::{TcpListener, TcpStream};
use std::io::Read;

use crate::models::config::Config;
use crate::services::copy::CopyService;
use crate::client::requests::*;

pub struct Client {
    config: Config,
    copy_service: CopyService,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            config: config.clone(), 
            copy_service: CopyService::new(config),
        }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("Error accepting connection: {}", e),
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        // todo: change this with some resizable structure.
        let mut buffer = vec![0u8; 4096];

        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received message: {}", request);

            let parsed = parse_request(&request.into_owned());
            println!("{:?}", parsed);
        }
    }
} 