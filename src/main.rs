use bytes::{BufMut, BytesMut};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut request = [0; 12];
                stream
                    .read_exact(&mut request)
                    .expect("12 bytes to be sent");

                let message_size: i32 = 0;
                let mut message = BytesMut::new();
                message.put_i32(message_size);
                message.put(&request[8..]);

                let _ = stream.write(&message[..]);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
