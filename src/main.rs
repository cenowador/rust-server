fn main() {
    let server = Server::new("localhost:3000".to_string());
    server.run();
}

struct Server {
    addr: String
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    fn run(self) {
        println!("Server started on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT
}