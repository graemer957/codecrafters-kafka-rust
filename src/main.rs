use bytes::{Buf, BufMut, Bytes};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

#[repr(i16)]
enum ErrorCode {
    UnsupportedVersion = 35,
}

const API_VERSIONS_API: i16 = 18;
const VERSIONS_API_MAX_VERSION: i16 = 4;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer);

                let mut request = Bytes::from(Vec::from(buffer));
                request.get_i32(); // message_size
                let request_api_key = request.get_i16();
                let request_api_version = request.get_i16();
                let correlation_id = request.get_i32();

                println!("request_api_key: {request_api_key}");
                println!("request_api_version: {request_api_version}");
                println!("correlation_id: {correlation_id}");

                let message_size: i32 = 0;
                let mut message = vec![];
                message.put_i32(message_size);
                message.put_i32(correlation_id);

                #[allow(clippy::single_match)]
                match request_api_key {
                    API_VERSIONS_API => {
                        if !(0..=VERSIONS_API_MAX_VERSION).contains(&request_api_version) {
                            message.put_i16(ErrorCode::UnsupportedVersion as i16);
                        }
                    }
                    _ => {}
                };

                let _ = stream.write_all(&message[..]);
            }
            Err(e) => println!("error: {e}"),
        }
    }
}
