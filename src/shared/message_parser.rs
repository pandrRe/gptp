pub use super::{Buffer, Message};
pub use super::constants::{STD_BUFFER_SIZE, MSG_TYPE_RANGE, HEADER_SIZE, MSG_DATA_POS};

pub struct MessageParser<'a> {
    buffer: &'a Buffer,
    current_position: usize
}

impl<'a> MessageParser<'a> {
    pub fn from(buffer: &Buffer) -> MessageParser {
        MessageParser {
            buffer,
            current_position: 0
        }
    }
}

impl<'a> Iterator for MessageParser<'a> {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        if self.current_position >= self.buffer.data.len() - 1 {
            return None;
        }

        let buffer_slice = &self.buffer.data[self.current_position..];

        if let Ok(message_type) = Message::get_type_from_buffer(buffer_slice) {
            let data_length = Message::get_size_from_buffer(buffer_slice);
            let message_data = &buffer_slice[HEADER_SIZE..HEADER_SIZE + data_length];
            self.current_position += HEADER_SIZE + data_length;

            return Some(Message::new(message_type, message_data));
        }

        return None;
    }
}