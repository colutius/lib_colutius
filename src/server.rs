#![allow(dead_code)]
#![allow(unused_variables)]
use std::net::TcpStream;
pub struct Server {
    host: String,
    port: String,
    nick: String,
    user: String,
}
impl Server {
    pub fn new(host: String, port: String, nick: String, user: String) -> Server {
        Server {
            host,
            port,
            nick,
            user,
        }
    }
    pub fn connect(&self) {
        let mut host = self.host.clone();
        let port = self.port.clone();
        host.push(':');
        host.push_str(&port);
        let stream = TcpStream::connect(host).expect("err");
    }
}
