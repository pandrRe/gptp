use super::MessageType;
use super::Buffer;
use super::ToRange;

const MSG_TYPE_RANGE: (usize, usize) = (0, 3);
const MSG_LEN_RANGE: (usize, usize) = (3, 4);
const MSG_DATA_POS: usize = 76;

pub struct Message<'a> {
    pub type_: MessageType,
    pub length: usize,
    pub data: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(type_: MessageType, length: usize, data: &[u8]) -> Message {
        Message {
            type_,
            length,
            data
        }
    }

    pub fn from_buffer(buffer: &Buffer) -> Message {
        let data_length_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, buffer.data[4], buffer.data[3]];
        let data_length: usize = usize::from_be_bytes(data_length_bytes);

        Message {
            type_: MessageType::from_str(std::str::from_utf8(&buffer.data[MSG_TYPE_RANGE.to_range()]).unwrap()).unwrap(),
            length: data_length,
            data: &buffer.data[MSG_DATA_POS..MSG_DATA_POS + data_length]
        }
    }
}