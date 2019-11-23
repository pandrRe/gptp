use std::io::prelude::*;
use std::net::TcpListener;
use crate::shared::Buffer;

pub struct Server {
    listener: TcpListener,
    buffer: Buffer,
}

impl Server {
    pub fn boot(port: i32) -> std::io::Result<Server> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port));

        match listener {
            Ok(listener) => Ok(Server { listener, buffer: Buffer::gptp_standard() }),
            Err(err) => Err(err)
        }
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream.read(&mut self.buffer.data).unwrap();
                    println!("{}", String::from_utf8_lossy(&self.buffer.data[..]));
                }
                Err(err) => {
                    panic!("An error has ocurred.\n{:?}", err)
                }
            }
        }   
    }
}