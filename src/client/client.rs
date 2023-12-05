use std::net::{TcpListener, TcpStream};
use std::io::Read;

use crate::models::config::Config;
use crate::services::copy::CopyService;
use crate::client::requests::*;
use crate::client::handlers::*;

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

    pub fn listen(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("Error accepting connection: {}", e),
            }
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        // todo: change this with some resizable container
        let mut buffer = vec![0u8; 4096];

        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("{:?}", self.handle_request(&request));
        }
    }

    fn handle_request(&mut self, request: &str) -> Option<String> {
        match parse_request(request) {
            Ok(parsed_request) => {
                let response = match parsed_request {
                    AnyRequest::Copy(copy_request) => 
                        handle_copy(copy_request, &mut self.copy_service),
                    AnyRequest::Suspend(suspend_request) => (),
                    AnyRequest::Cancel(cancel_request) => (),
                    AnyRequest::Progress(progress_request) => (),
                    AnyRequest::List(list_request) => (),
                };

                // todo
                Some("".to_owned())
            }
            Err(_) => None,
        }
    }
}