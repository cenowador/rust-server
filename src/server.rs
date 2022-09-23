use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler{
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Error while parsing request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    pub fn run(self, mut handler: impl Handler) {
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
                                    handler.handle_request(&request)
                                },
                                Err(e) =>{
                                    handler.handle_bad_request(&e)
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