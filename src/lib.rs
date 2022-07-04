mod message;
#[cfg(test)]
mod tests {
    use crate::message;
    #[test]
    fn test_new_message() {
        let new_message = message::Message::new()
            .on_server("irc.libera.chat".to_string())
            .my_nick("hgy".to_string())
            .raw_msg("PING hhhhhhhhhhhhhhh".to_string())
            .parse();
        new_message.show();
    }
    #[test]
    fn test() {
        println!("hhh");
    }
}
