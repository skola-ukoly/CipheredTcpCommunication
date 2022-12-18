use crate::error::*;
use crate::middleware::Middleware;

use std::net::TcpStream;
use std::io::{Read, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

use regex::Regex;

/// stores a tcp stream thread safely, use it to read/write to it
pub struct Stream {
    pub stream: Arc<Mutex<TcpStream>>,
}

impl Stream {
    /// Takes an ip address and port, validates it and tries to connect to tcp server
    pub fn connect_to_server(addr: String, port: u16) -> Result<Self> {
        if !validate_ip_address(&addr) {
            return Err(CommunicationError::Generic("IP address is invalid".to_string()));
        };

        let stream = match TcpStream::connect(format!("{}:{}", addr, port)) {
            Ok(stream) => stream,
            Err(err) => return Err(CommunicationError::Generic("cant connect to server".to_string()))
        };

        Ok(Self { 
            stream: Arc::new(Mutex::new(stream))
        })
    }

}

fn validate_ip_address(address: &String) -> bool {
    let address_matching_regex = match Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$") {
        Ok(regex) => regex,
        Err(_) => return false,
    };

    address_matching_regex.is_match(&address)
}

pub fn create_reader_thread(stream: &Stream, middleware: &Middleware) -> thread::JoinHandle<()> {
    let stream_mutex = Arc::clone(&stream.stream);
    let mut buf = vec![0u8;1024];

    thread::spawn(move || {
        loop {
            if let Ok(mut stream_locked) = stream_mutex.lock() {
                stream_locked.read(&mut buf);

                println!("{buf:#?}");
            }
        }
    })
}
