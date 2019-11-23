use std::fmt;

const STD_BUFFER_SIZE: usize = 1100;

pub struct Buffer {
    pub data: [u8; STD_BUFFER_SIZE]
}

impl Buffer {
    pub fn gptp_standard() -> Buffer {
        Buffer {
            data: [0; STD_BUFFER_SIZE]
        }
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}