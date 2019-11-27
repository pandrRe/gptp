use std::io::{stdin};

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

    loop {
        let mut input_buffer = String::new();
        stdin().read_line(&mut input_buffer).unwrap();

        if input_buffer == "!!!\n" {
            break;
        }

        let message = Message::new(MessageType::TextData, input_buffer.trim().as_bytes());
        connection.write_message_to_buffer(&message);
        connection.send_message_on_buffer();
    }

    handle.join().unwrap();
}
