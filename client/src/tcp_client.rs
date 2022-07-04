use std::io::{Read, Write};
use std::net::TcpStream;
use common::model::JsonMessage;

pub struct TcpClient {
    tcp_stream: TcpStream,
}

impl TcpClient {
    pub fn initialize() -> TcpClient {
        let stream_port = TcpStream::connect("localhost:7878");
        let tcp_stream = match stream_port {
            Ok(res) => res,
            Err(err) => panic!("Error: {err}")
        };
        TcpClient {
            tcp_stream
        }
    }

    pub fn send(&mut self, json_message: &JsonMessage) -> JsonMessage {
        self.send_message(json_message);
        self.waiting_response()
    }

    pub fn send_message(&mut self, message: &JsonMessage) {
        let json_message = serde_json::to_string(&message).unwrap();
        let size = json_message.len() as u32;
        self.tcp_stream.write(&size.to_be_bytes()).unwrap();
        self.tcp_stream.write_all(&json_message.as_bytes()).unwrap();
    }

    pub fn waiting_response(&mut self) -> JsonMessage {
        let mut response_buffer = [0u8; 4];
        println!("I m waiting");
        loop {
            let result: Result<(), _> = self.tcp_stream.read_exact(&mut response_buffer);
            if result.is_ok() {
                break;
            }
        }

        let size: u32 = u32::from_be_bytes(response_buffer);
        let mut response: Vec<u8> = vec![0; size as usize];

        self.tcp_stream.read_exact(&mut response).unwrap();
        let str_res = std::str::from_utf8(&response).unwrap();
        let json_message: JsonMessage = serde_json::from_str(&str_res).unwrap();
        json_message
    }
}