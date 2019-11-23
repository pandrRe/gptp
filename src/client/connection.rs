use std::io::prelude::*;
use std::net::TcpStream;
use crate::shared::Buffer;

const HEADER_SIZE: u8 = 76;

pub struct Connection {
    stream: TcpStream,
    buffer: Buffer, 
    data_length: usize
}

impl Connection {
    pub fn establish(ip: &str, port: i32) -> Connection {
        return Connection {
            stream: TcpStream::connect(format!("{}:{}", ip, port)).unwrap(),
            buffer: Buffer::gptp_standard(),
            data_length: 0
        }
    }

    pub fn write_to_buffer(&mut self, data: &[u8]) {
        if data.len() > 1024 {
            panic!("Data must have up to 1024 bytes.");
        }

        self.data_length = data.len();
        for (i, &byte) in data.iter().enumerate() {
            self.buffer.data[i] = byte;
        }
    }

    pub fn send_data_on_buffer(&mut self) {
        self.stream.write(&self.buffer.data[0..self.data_length]);
    }
}