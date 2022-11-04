use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;
use std::net::TcpListener;

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
    pub fn run(self) -> ! {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", &self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Create buffer for io::read, 1024 in size. Takes u8, so just over 1kb
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => { 
                            println!("Request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("failed to parse request {}", e),
                            }
                            let res: &Result<Request, _> = &buffer[..].try_into();


                        },

                        Err(e) => println!("Error: {}", e)
                    }
                }
                Err(e) => println!("Err: {}", e),
            }
        }
    }
}
