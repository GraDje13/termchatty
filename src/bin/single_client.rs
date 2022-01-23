use std::env;
use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const ARG_NUMBER: usize = 3;
const SOH: u8 = 1;
const EOT: u8 = 4;
const EXIT_COMMAND: &str = "!exit";

fn main() -> Result<(), &'static str> {
    // griefing silly windows users
    if cfg!(windows) {
        for _i in 1..100 {
            println!("install linux");
        }
    }

    // collect the command line arguments
    let mut args: Vec<String> = Vec::with_capacity(ARG_NUMBER);
    args = env::args().collect();

    if args.len() != ARG_NUMBER {
        return Err("wrong number of arguments");
    }

    let socket_address = args.remove(2);
    let remote_adress = args.remove(1);

    // make the listener
    let listener = match TcpListener::bind(socket_address) {
        Ok(list) => list,
        Err(e) => {
            println!("{:?}", e);
            return Err("Failed to create listener");
        }
    };

    // variable used to keep track of connection status, used so it does not try to disconnect when
    // other user has already done so
    let connected = Arc::new(Mutex::new(false));
    let connected_thread = Arc::clone(&connected);

    let listener_thread = thread::spawn(move || {
        loop {
            println!("awaiting remote connection...");
            let (mut socket, address) = match listener.accept() {
                Ok((sock, addr)) => {
                    println!("Connection with: {:?}", addr);
                    *connected_thread.lock().unwrap() = true;
                    (sock, addr)
                }
                Err(_e) => panic!("Connection failed"),
            };

            loop {
                // wait till a message comes
                let mut start_buffer = [0];

                socket.read_exact(&mut start_buffer);

                // see if other user has send disconnect message
                if start_buffer[0] == EOT {
                    println!("remote disconnected");
                    *connected_thread.lock().unwrap() = false;
                    break;
                }

                // if a message is detected, read it and display it
                if start_buffer[0] == SOH {
                    println!("{}", message_read(&mut socket));
                }

                //so it does not suck to much cpu
                thread::sleep(Duration::from_millis(10));
            }
        }
    });

    let mut remote_socket = connect_until_success(&remote_adress);

    loop {
        let mut message_buffer = String::from("");

        stdin().read_line(&mut message_buffer).unwrap();
        message_buffer.pop();

        if message_buffer == EXIT_COMMAND {
            if *connected.lock().unwrap() {
                disconnect(&mut remote_socket);
            }
            break;
        } else {
            message_send(&message_buffer, &mut remote_socket);
        }
    }

    Ok(())
}

fn message_send(message: &str, socket: &mut TcpStream) {
    let message_length = message.len();
    socket.write_all(&[SOH]).unwrap();
    socket.write_all(&message_length.to_le_bytes()).unwrap();
    socket.write_all(message.as_bytes()).unwrap();
}

fn message_read(socket: &mut TcpStream) -> String {
    let mut length_buffer = [0; 8];
    socket.read_exact(&mut length_buffer).unwrap();

    let mut message_buffer = vec![0; usize::from_le_bytes(length_buffer)];
    socket.read_exact(&mut message_buffer).unwrap();

    String::from_utf8(message_buffer).unwrap()
}

fn connect_until_success(addr: &str) -> TcpStream {
    loop {
        if let Ok(socket) = TcpStream::connect(addr) {
            return socket;
        } else {
            println!("user not online, trying again in a second");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn disconnect(socket: &mut TcpStream) {
    socket.write_all(&[EOT]).unwrap();
    socket.shutdown(Shutdown::Both).unwrap();
}
