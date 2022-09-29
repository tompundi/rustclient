use std::net::{TcpListener, TcpStream};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Server listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();


        loop {

            match listener.accept() {
                Ok((stream, _)) => {
                    let a = 5;
                    println!("OK!")
                }

                Err(e) => {
                    println!("Failed to establish a connection {}", e)
                }
            }

        }

    }
}
