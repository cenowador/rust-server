use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};

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
        println!("Server started on {}", &self.addr);
        loop{
            match listener.accept() {
                Ok((mut stream, _)) =>{
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) =>{
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) =>{
                                    Response::new(StatusCode::Ok, Some("<h1>Hello, World!</h1>".to_string()))
                                },
                                Err(e) =>{
                                    println!("Error while parsing request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            
                            if let Err(e) = response.send(&mut stream){
                                println!("Error while sending response: {}", e);
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