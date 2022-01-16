use message_io::network::Transport;
use message_io::node::{self};
use std::{thread, time};

const ADDRESS: &str = "127.0.0.1:3042";

fn main() {
    let (handler, _listener) = node::split::<()>();

    let (server, _) = handler
        .network()
        .connect(Transport::FramedTcp, ADDRESS)
        .unwrap();

    handler.network().send(server, "bruh moment".as_bytes());
    thread::sleep(time::Duration::from_secs(1));
    handler
        .network()
        .send(server, "another buh moment".as_bytes());
}
