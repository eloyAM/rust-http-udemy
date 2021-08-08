fn main() {
    let get = Method::GET("Hello".to_string());
    let delete = Method::DELETE(69);
    let post = Method::POST;
    // Not possible casting
    // an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    dbg!(get as String); dbg!(delete as u64); dbg!(post as u64);
    
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
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}