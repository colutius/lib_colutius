#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::net::TcpStream;
pub struct Server {
    nick: String,
    user: String,
    stream: TcpStream,
}
impl Server {
    pub fn new(host: &str, port: &str, nick: &str, user: &str) -> Server {
        Server {
            nick: nick.to_string(),
            user: user.to_string(),
            stream: TcpStream::connect(format!("{}:{}", host, port)).expect("connect fail!"),
        }
    }
}
