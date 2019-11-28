use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use crate::shared::Buffer;
use crate::shared::MessageType;
use crate::shared::Message;
use crate::shared::constants::{HEADER_SIZE, MSG_LEN_RANGE};

pub struct Connection {
    stream: TcpStream,
    buffer: Buffer, 
    data_length: usize
}

impl Connection {
    pub fn establish(ip: &str, port: i32) -> Connection {
        println!("[Connection to server on {}:{} established]", ip, port);
        return Connection {
            stream: TcpStream::connect(format!("{}:{}", ip, port)).unwrap(),
            buffer: Buffer::gptp_standard(),
            data_length: 0
        }
    }

    fn set_message_type(&mut self, message_type: MessageType) {
        for (i, &byte) in message_type.as_writable_to_header() {
            self.buffer.data[i] = byte;
        }
    }

    fn set_message_length_from_current_data(&mut self) {
        for (i, &byte) in self.data_length.to_be_bytes().iter().enumerate() {
            self.buffer.data[i + MSG_LEN_RANGE.0] = byte;
        }
    }

    fn write_data_to_buffer(&mut self, data: &[u8]) {
        if data.len() > 1024 {
            panic!("Data must have up to 1024 bytes.");
        }

        self.data_length = data.len();
        for (i, &byte) in data.iter().enumerate() {
            self.buffer.data[i + HEADER_SIZE] = byte;
        }
    }

    pub fn write_message_to_buffer(&mut self, message: &Message) {
        self.set_message_type(message.type_);
        self.write_data_to_buffer(&message.data);
        self.set_message_length_from_current_data();
    }

    pub fn send_message_on_buffer(&mut self) {
        self.stream.write(&self.buffer.data[0..HEADER_SIZE + self.data_length]).unwrap();
        self.stream.flush().unwrap();
    }

    fn send_close_message(&mut self) {
        let message = Message::new(MessageType::End, &[1]);
        self.write_message_to_buffer(&message);
        self.send_message_on_buffer();
    }

    pub fn close(&mut self) {
        self.send_close_message();
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}