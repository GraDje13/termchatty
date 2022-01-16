use message_io::network::{NetEvent, Transport};
use message_io::node::{self};

const ADDRESS: &str = "0.0.0.0:3042";

fn main() {
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
