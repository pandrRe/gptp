use std::io::{stdin};
use std::thread;

use gptp::Server;
use gptp::client::Connection;
use gptp::shared::{Message, MessageType};

fn main() {
    let mut server = Server::boot(8080).unwrap();

    let handle = thread::spawn(move || {
        server.listen();
    });

    let mut connection: Option<Connection> = None;
    let mut input_buffer = String::new();

    loop {
        stdin().read_line(&mut input_buffer).unwrap();

        if input_buffer == "!!!\n" {
            break;
        }

        if input_buffer.starts_with("@") {
            connection = Some(Connection::establish("127.0.0.1", input_buffer[1..5].parse::<i32>().unwrap()));
            break;
        }
    }

    input_buffer.clear();
    if let Some(mut conn) = connection {
        loop {
            stdin().read_line(&mut input_buffer).unwrap();
            let message = Message::new(MessageType::Request, input_buffer.trim().as_bytes());

            if input_buffer == "!!!\n" {
                break;
            }

            conn.write_message_to_buffer(&message);
            conn.send_message_on_buffer();

            input_buffer.clear();
        }
    }

    handle.join().unwrap();
}
