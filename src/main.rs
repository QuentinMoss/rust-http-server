fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // Structs: 'Self' is an alias for the name of struct
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }
    // 'self' follows same ownership as normal variables. Takes ownership of the entire struct.
    // Struct will be deallocated when run exits.
    // If you do not want to deallocate the struct after exit, use reference
    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}
