use bytes::{BufMut, BytesMut};
use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    let message_size: i32 = 0;
    let correlation_id: i32 = 7;
    let mut message = BytesMut::new();
    message.put_i32(message_size);
    message.put_i32(correlation_id);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let _ = stream.write(&message[..]);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
