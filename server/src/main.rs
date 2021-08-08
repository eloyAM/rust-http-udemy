fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    dbg!(get as u64); dbg!(delete as u64); dbg!(post as u64);
    let server = Server::new("127.0.0.1:8080".to_string());
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

    fn run (self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}