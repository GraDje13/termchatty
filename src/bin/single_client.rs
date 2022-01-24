// TODO fix bug with reconnecting

use anyhow::{anyhow, Context};
use std::env;
use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const ARG_NUMBER: usize = 3;
const SOH: u8 = 1;
const EOT: u8 = 4;
const EXIT_COMMAND: &str = "!exit";

fn main() -> anyhow::Result<()> {
    // griefing silly windows users
    if cfg!(windows) {
        for _i in 1..100 {
            println!("install linux");
        }
    }

    // collect the command line arguments
    let mut args: Vec<String> = env::args().collect();

    if args.len() != ARG_NUMBER {
        return Err(anyhow!("failed to create listener"));
    }

    let socket_address = args.remove(2);
    let remote_adress = args.remove(1);

    // make the listener
    let listener = TcpListener::bind(socket_address).context("failed to make listener")?;

    let _listener_thread = thread::spawn(move || -> anyhow::Result<()> {
        loop {
            println!("awaiting remote connection...");
            let (mut socket, _address) = match listener.accept() {
                Ok((sock, addr)) => {
                    println!("Connection with: {:?}", addr);
                    (sock, addr)
                }
                Err(_e) => return Err(anyhow!("Accepting the connection failed")),
            };

            loop {
                // wait till a message comes
                let mut start_buffer = [0];

                socket.read_exact(&mut start_buffer)?;

                // see if other user has send disconnect message
                if start_buffer[0] == EOT {
                    println!("remote disconnected");
                    break;
                }

                // if a message is detected, read it and display it
                if start_buffer[0] == SOH {
                    let message = message_read(&mut socket)?;
                    println!("{}", message);
                }

                //so it does not suck to much cpu
                thread::sleep(Duration::from_millis(10));
            }
        }
    });

    let mut remote_socket = connect_until_success(&remote_adress);

    loop {
        let mut message_buffer = String::from("");

        stdin().read_line(&mut message_buffer)?;
        message_buffer.pop();

        if message_buffer == EXIT_COMMAND {
            disconnect(&mut remote_socket)?;
            break;
        } else {
            message_send(&message_buffer, &mut remote_socket)?;
        }
    }

    Ok(())
}

fn message_send(message: &str, socket: &mut TcpStream) -> anyhow::Result<()> {
    let message_length = message.len();
    socket.write_all(&[SOH])?;
    socket.write_all(&message_length.to_le_bytes())?;
    socket.write_all(message.as_bytes())?;
    Ok(())
}

fn message_read(socket: &mut TcpStream) -> anyhow::Result<String> {
    let mut length_buffer = [0; 8];
    socket.read_exact(&mut length_buffer)?;

    let mut message_buffer = vec![0; usize::from_le_bytes(length_buffer)];
    socket.read_exact(&mut message_buffer)?;

    Ok(String::from_utf8(message_buffer.to_vec())?)
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

fn disconnect(socket: &mut TcpStream) -> anyhow::Result<()> {
    socket.write_all(&[EOT])?;
    socket.shutdown(Shutdown::Both)?;
    Ok(())
}
