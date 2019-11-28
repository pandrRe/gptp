use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use crate::shared::{Buffer, MessageParser, Message, MessageType};

pub struct Server {
    listener: TcpListener,
    buffer: Buffer,
}

impl Server {
    pub fn boot(port: i32) -> std::io::Result<Server> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port));

        match listener {
            Ok(listener) => {
                println!("[Server listening on port {}.]\n", port);
                Ok(Server { listener, buffer: Buffer::gptp_standard() })
            },
            Err(err) => Err(err)
        }
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("\n[Connection from {} established.]\n", stream.peer_addr().unwrap().ip());

                    while let Ok(bytes_read) = stream.read(&mut self.buffer.data) {
                        if bytes_read == 0 {
                            break;
                        }

                        for message in MessageParser::from(&self.buffer) {
                            self.handle_message(&stream, &message);
                        }
                    }
                }
                Err(err) => {
                    panic!("An error has ocurred.\n{:?}", err)
                }
            }
        }
    }
    
    fn handle_message(&self, stream: &TcpStream, message: &Message) {
        match message.type_ {
            MessageType::TextData => {
                println!("[{}]: {}", stream.peer_addr().unwrap().ip(), String::from_utf8_lossy(&message.data));
            },
            MessageType::End => {
                stream.shutdown(Shutdown::Read).unwrap();
                println!("\n[Connection closed.]\n");
            }
            _ => {}
        }
    }
}