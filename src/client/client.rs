use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

use crate::client::handlers::*;
use crate::client::requests::*;
use crate::models::job::Job;
use crate::services::storage::StorageService;

pub struct Client {
    storage: Arc<RwLock<StorageService>>,
    sender: Sender<Job>,
}

impl Client {
    pub fn new(storage: Arc<RwLock<StorageService>>, sender: Sender<Job>) -> Self {
        Client {
            storage,
            sender,
        }
    }

    pub fn listen(&mut self) {        
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_stream(stream),
                Err(e) => eprintln!("Error accepting connection: {}", e),
            }
        }
    }

    fn handle_stream(&self, mut stream: TcpStream) {
        // todo: change this with some resizable container
        let mut buffer = vec![0u8; 4096];

        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            self.send_response(&mut stream,&self.handle_request(&request));
        }
    }

    fn handle_request(&self, request: &str) -> Option<String> {
        let response = match parse_request(request) {
            Ok(parsed_request) => {
                match parsed_request {
                    AnyRequest::Copy(copy_request) => 
                        handle_copy(copy_request, self.sender.clone()),
                    AnyRequest::Suspend(suspend_request) =>
                        handle_suspend(suspend_request, self.storage.clone()),
                    AnyRequest::Cancel(cancel_request) =>
                        handle_cancel(cancel_request, self.storage.clone()),
                    AnyRequest::Progress(progress_request) => 
                        handle_progress(progress_request, self.storage.clone()),
                    AnyRequest::List(_) => 
                        handle_list(self.storage.clone()),
                }
            }, 
            Err(err) => handle_error(err),
        };

        match response {
            Ok(str) => Some(str), 
            Err(err) => Some(err.to_string()),
        }
    }

    fn send_response(&self, stream: &mut TcpStream, response: &Option<String>) {
        if let Some(response_str) = response {
            if let Err(err) = stream.write_all(response_str.as_bytes()) {
                eprintln!("Error sending response: {}", err);
            }
        }
    }
}
