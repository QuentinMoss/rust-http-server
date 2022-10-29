pub struct Server {
    addr: String,
}

impl Server {
    // Structs: 'Self' is an alias for the name of struct
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }
    // 'self' follows same ownership as normal variables. Takes ownership of the entire struct.
    // Struct will be deallocated when run exits.
    // If you do not want to deallocate the struct after exit, use reference
    pub fn run(self) {
        println!("Listening on {}", self.addr)
    }
}
