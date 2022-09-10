use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop{
            match listener.accept() {
                Ok((mut stream, addr)) =>{
                    println!("Server started on {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) =>{
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) =>{},
                                Err(e) =>{
                                    println!("Error while parsing request: {}", e);
                                }
                            }
                        },
                        Err(e) =>{
                            println!("Error while reading from connection: {}", e);
                        }
                    }
                },
                Err(e) =>{
                    println!("Error while stabilishing connection! {}", e);
                }
            }
        }
    }
}