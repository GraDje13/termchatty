use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").expect("can't connect here");

    let mut message = String::from("hello");
    message_send(message, &mut socket);

    let mut message = String::from("prok is cool");
    message_send(message, &mut socket);

    let mut message = String::from("Assuming the necessary assumptions..");
    message_send(message, &mut socket);

    disconnect(&mut socket);
}

fn message_send(message: String, socket: &mut TcpStream) {
    let message_length = message.len() as u8;

    socket.write_all(&[1]).unwrap();
    socket.write_all(&[message_length]).unwrap();
    socket.write_all(message.as_bytes()).unwrap();
}

fn disconnect(socket: &mut TcpStream) {
    socket.write_all(&[4]).unwrap();

    socket
        .shutdown(std::net::Shutdown::Both)
        .expect("could not shutdown connection");
}
