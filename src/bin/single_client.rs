use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, io};

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

    // collect the command line arguments and pop them into the
    // relevant variables.
    let mut args: Vec<String> = Vec::with_capacity(ARG_NUMBER);
    args = env::args().collect();

    args.reverse();
    args.pop();

    let remote_adress = match args.pop() {
        Some(arg) => arg,
        None => {
            return Err("Please enter a remote adress");
        }
    };

    let socket_address = match args.pop() {
        Some(arg) => arg,
        None => {
            return Err("Pease enter were you want your socket");
        }
    };

    println!("Attempting to create listener on: {}", socket_address);

    let listener = match TcpListener::bind(socket_address) {
        Ok(list) => list,
        Err(e) => {
            println!("{:?}", e);
            return Err("Failed to create listener");
        }
    };

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
                    let mut length_buffer = [0];
                    socket.read_exact(&mut length_buffer).unwrap();

                    let mut message_buffer = vec![0; length_buffer[0] as usize];
                    socket.read_exact(&mut message_buffer).unwrap();

                    println!(
                        "{}> {}",
                        address,
                        String::from_utf8_lossy(message_buffer.as_ref())
                    );
                }

                //so it does not suck to much cpu
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    });

    let stdin = io::stdin();
    let mut enter_buffer = String::from("");

    println!("press enter to continue");
    stdin.read_line(&mut enter_buffer).unwrap();

    let mut remote_socket = TcpStream::connect(remote_adress).unwrap();

    loop {
        let mut message_buffer = String::from("");
        print!("you>");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut message_buffer).unwrap();
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
    let message_length = message.len() as u8;
    // TODO: increase character limit from 255 to 2^64
    socket.write_all(&[SOH]).unwrap();
    socket.write_all(&[message_length]).unwrap();
    socket.write_all(message.as_bytes()).unwrap();
}

fn disconnect(socket: &mut TcpStream) {
    socket.write_all(&[EOT]).unwrap();
    socket.shutdown(Shutdown::Both).unwrap();
}
