use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // Define a default method. This can be overriden
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }

}

impl Server {
    // Structs: 'Self' is an alias for the name of struct
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    // 'self' follows same ownership as normal variables. Takes ownership of the entire struct.
    // Struct will be deallocated when run exits.
    // If you do not want to deallocate the struct after exit, use reference

    pub fn run(self, mut handler: impl Handler) -> ! {
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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handle_request(&request)
                                }

                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send resposne {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                }

                Err(e) => println!("Failed to accept connection: {}", e),
            }
        }
    }
}
