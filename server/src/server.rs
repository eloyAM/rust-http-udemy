pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run (self) {
        println!("Listening on {}", self.addr);
        let listener: std::net::TcpListener = std::net::TcpListener::bind(&self.addr).unwrap();

        loop {
            listener.accept();
        }

        let tuple: (&str, u64) = ("hello", 2);
    }
}