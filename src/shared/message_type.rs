use std::io::{Error, ErrorKind};

#[derive(Copy, Clone)]
pub enum MessageType {
    Request,
    AcceptRequest,
    DenyRequest,
    TextData,
    RequestFile,
    AcceptFile,
    DenyFile,
    FileData
}

impl MessageType {
    pub fn from_str(string: &str) -> std::io::Result<MessageType> {
        match string {
            "REQ" => Ok(MessageType::Request),
            "ACP" => Ok(MessageType::AcceptRequest),
            "DNY" => Ok(MessageType::DenyRequest),
            "TXT" => Ok(MessageType::TextData),
            "RQF" => Ok(MessageType::RequestFile),
            "ACF" => Ok(MessageType::AcceptFile),
            "DNF" => Ok(MessageType::DenyFile),
            "FLE" => Ok(MessageType::FileData),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid message type."))
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            &MessageType::Request => "REQ",
            &MessageType::AcceptRequest => "ACP",
            &MessageType::DenyRequest => "DNY",
            &MessageType::TextData => "TXT",
            &MessageType::RequestFile => "RQF",
            &MessageType::AcceptFile => "ACF",
            &MessageType::DenyFile => "DNF",
            &MessageType::FileData => "FLE"
        }
    }

    pub fn as_writable_to_header(&self) -> std::iter::Enumerate<std::slice::Iter<'_, u8>> {
        return self.as_str().as_bytes().iter().enumerate();
    }
}