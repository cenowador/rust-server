use super::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebserverHandler{
    public_path: String
}

impl WebserverHandler{
    pub fn new(public_path: String) -> Self{
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String>{
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(canonical) =>{
                if canonical.starts_with(fs::canonicalize(&self.public_path).unwrap()) {
                    fs::read_to_string(canonical).ok()
                }
                else{
                    println!("directory traversal {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl Handler for WebserverHandler{
    fn handle_request(&mut self, request: &Request) -> Response{
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/hi" => Response::new(StatusCode::Ok, self.read_file("hi.html")),
                    path => {
                        match self.read_file(path){
                            Some(content)=> Response::new(StatusCode::Ok, Some(content)),
                            None => Response::new(StatusCode::NotFound, None)
                        }
                    }
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}