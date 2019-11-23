use gptp::Server;
use gptp::client::Connection;

use std::thread;

fn main() {
    let mut server = Server::boot(8080).unwrap();

    let handle = thread::spawn(move || {
        server.listen();
    });

    let mut connection = Connection::establish("127.0.0.1", 8080);
    connection.write_to_buffer(&[1, 2, 3]);
    connection.send_data_on_buffer();
    handle.join().unwrap();
}
