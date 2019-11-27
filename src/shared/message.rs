use super::MessageType;
use super::Buffer;
use super::ToRange;
use super::constants::{MSG_TYPE_RANGE, MSG_DATA_POS, MSG_LEN_RANGE};
use std::io::{Error, ErrorKind};

pub struct Message {
    pub type_: MessageType,
    pub length: usize,
    pub data: Box<[u8]>,
    pub is_complete: bool
}

impl<'a> Message {
    pub fn new(type_: MessageType, data: &[u8]) -> Message {
        Message {
            type_,
            length: data.len(),
            data: Box::from(data),
            is_complete: true
        }
    }

    pub fn get_type_from_buffer(raw_buffer: &[u8]) -> std::io::Result<MessageType> {
        let message_type_str = std::str::from_utf8(&raw_buffer[MSG_TYPE_RANGE.to_range()]);

        if let Err(_) = message_type_str {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid message type."));
        }

        return MessageType::from_str(message_type_str.unwrap());
    }

    pub fn get_size_from_buffer(raw_buffer: &[u8]) -> usize {
        let mut length_bytes: [u8; 8] = [0; 8];
        length_bytes.copy_from_slice(&raw_buffer[MSG_LEN_RANGE.to_range()]);
        return usize::from_be_bytes(length_bytes);
    }

    pub fn from_buffer(buffer: &Buffer) -> Message {
        let data_length_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, buffer.data[4], buffer.data[3]];
        let data_length: usize = usize::from_be_bytes(data_length_bytes);

        Message {
            type_: MessageType::from_str(std::str::from_utf8(&buffer.data[MSG_TYPE_RANGE.to_range()]).unwrap()).unwrap(),
            length: data_length,
            data: Box::from(&buffer.data[MSG_DATA_POS..MSG_DATA_POS + data_length]),
            is_complete: true
        }
    }
}