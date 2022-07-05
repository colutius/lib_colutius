mod client;
mod message;
mod server;
mod session;
#[cfg(test)]
mod tests {
    use crate::message;
    use crate::server;
    #[test]
    fn test_new_message() {
        let _new_message = message::Message::new()
            .on_server("irc.libera.chat".to_string())
            .my_nick("hgy".to_string())
            .raw_msg("hgy!~hgy@192.168.0.1 PRIVMSG #linux :哈哈哈😃".to_string())
            .parse();
        println!("{:?}", _new_message);
        // _new_message.show();
    }
    #[test]
    fn test() {
        let _new_message = message::Message::new()
            .on_server("irc.libera.chat".to_string())
            .my_nick("hgy".to_string())
            .raw_msg("PING irc.libera.chat".to_string())
            .parse();
        println!("{:?}", _new_message);
        // _new_message.show();
    }
    #[test]
    fn test_server() {
        let _new_server = server::Server::new(
            "irc.libera.chat".to_string(),
            "6667".to_string(),
            "hgy".to_string(),
            "hgy".to_string(),
        );
        //new_server.connect();
    }
}
