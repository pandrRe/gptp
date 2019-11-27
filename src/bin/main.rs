use gptp::Server;
use gptp::client::Connection;
use gptp::shared::{Message, MessageType};

use std::thread;

fn main() {
    let mut server = Server::boot(8080).unwrap();

    let handle = thread::spawn(move || {
        server.listen();
    });

    let mut connection = Connection::establish("127.0.0.1", 8080);

    let message_data = b"test message 1";
    let message = Message::new(MessageType::TextData, message_data.len(), message_data);

    connection.write_message_to_buffer(&message);
    connection.send_message_on_buffer();

    handle.join().unwrap();
}
