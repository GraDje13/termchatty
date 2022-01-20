use std::io::{self, Read, Write};
use std::os::unix::net::UnixDatagram;
use std::thread;
use std::time::Duration;
const UNIX_SOCKET_PATH: &str = "/tmp/termchatty.sock";
const CHAR_LIMIT: usize = 2048;

fn main() {
    // create socket
    let unix_socket = match UnixDatagram::unbound() {
        Ok(sock) => sock,
        Err(e) => panic!("Failed to create socket: {:?}", e),
    };

    // connect to local server
    match unix_socket.connect(UNIX_SOCKET_PATH) {
        Ok(_) => println!("connected to socket"),
        Err(e) => panic!("failed to connect to socket: {:?}", e),
    }

    let mut input_buffer = [0; CHAR_LIMIT];
    let stdin = io::stdin();

    print!("select user to talk to");
    stdin.read_line(input_buffer).unwrap();

    input_buffer.pop();

    unix_socket.send("!switch".as_bytes()).unwrap();
    unix_socket.send(input_buffer.as_bytes()).unwrap();

    let mut recipient = String::from_utf8_lossy(input_buffer);

    loop {
        print!("to: {} > ", recipient);
        stdin.read_line(&mut input_buffer).unwrap();
        input_buffer.pop();

        if input_buffer == "!exit" {
            break;
        } else {
            unix_socket.send(input_buffer.as_bytes()).unwrap();
        }
    }
}
