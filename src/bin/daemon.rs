use message_io::network::{NetEvent, Transport};
use message_io::node::{self};
use std::fs;
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::thread;

const ADDRESS: &str = "0.0.0.0:3042";
const UNIX_SOCKET_PATH: &str = "/tmp/termchatty.sock";
const SWITCH_RECIPIENT_COMMAND: &str = "!switch";
const CHAR_LIMIT: usize = 2048;

fn main() {
    // prank
    if cfg!(windows) {
        loop {
            println!("install linux");
        }
    }

    let _unix_socket_thread = thread::spawn(|| {
        let unix_socket_path = Path::new(UNIX_SOCKET_PATH);

        if unix_socket_path.exists() {
            fs::remove_file(unix_socket_path).expect("failed to remove old socket file");
        }

        let unix_socket = match UnixDatagram::bind(unix_socket_path) {
            Ok(sock) => sock,
            Err(e) => panic!("Failed to bind socket: {:?}", e),
        };

        let mut unix_buffer = [0; CHAR_LIMIT];
        let mut recipient = String::from("");
        loop {
            let _data = unix_socket.recv(unix_buffer.as_mut_slice()).unwrap();

            if unix_buffer[0] == 33 {
                let command = String::from_utf8_lossy(&unix_buffer);

                if command == SWITCH_RECIPIENT_COMMAND {
                    unix_socket.recv(unix_buffer.as_mut_slice()).unwrap();
                    recipient = String::from_utf8(unix_buffer.to_vec()).unwrap();
                }
            } else {
                print!("to: {} >", recipient);
                print!(" {} \n", String::from_utf8_lossy(&unix_buffer));
            }
        }
    });

    let (handler, listener) = node::split::<()>();

    handler
        .network()
        .listen(Transport::FramedTcp, ADDRESS)
        .unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Accepted(_endpoint, _listener) => println!("Someone connected"),
        NetEvent::Message(_endpoint, data) => {
            println!("Received: {}", String::from_utf8_lossy(data))
        }
        _ => (),
    });
}
