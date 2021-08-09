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
            match listener.accept() {
                Ok((stream, _)) => {
                    let a = 5;
                    println!("Ok");
                }
                Err(e) => println!("Failed to establish the connection: {}", e)
            }
        }
    }
}