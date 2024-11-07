use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

#[derive(Debug)]
struct RequestHeader {
    _message_size: i32,
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
}

#[repr(i16)]
enum ErrorCode {
    UnsupportedVersion = 35,
}

const API_VERSIONS_API: i16 = 18;
const VERSIONS_API_MAX_VERSION: i16 = 4;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = BytesMut::zeroed(1024);
                let _ = stream.read(&mut buffer)?;

                let request_header = RequestHeader {
                    _message_size: buffer.get_i32(),
                    request_api_key: buffer.get_i16(),
                    request_api_version: buffer.get_i16(),
                    correlation_id: buffer.get_i32(),
                };
                dbg!(&request_header);

                buffer.clear();
                buffer.put_i32(0); // TODO: Not currently calculating `message_size`
                buffer.put_i32(request_header.correlation_id);

                #[allow(clippy::single_match)]
                match request_header.request_api_key {
                    API_VERSIONS_API => {
                        if !(0..=VERSIONS_API_MAX_VERSION)
                            .contains(&request_header.request_api_version)
                        {
                            buffer.put_i16(ErrorCode::UnsupportedVersion as i16);
                        }
                    }
                    _ => {}
                };

                stream.write_all(&buffer[..])?;
            }
            Err(e) => println!("error: {e}"),
        }
    }

    Ok(())
}
