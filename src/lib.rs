mod message;
#[cfg(test)]
mod tests {
    use crate::message;
    #[test]
    fn test_new_message() {
        let new_message = message::Message::new()
            .raw("hello world")
            .main_message("world")
            .from(message::Who::Server("liberachat".to_string()))
            .to(message::Who::People("me".to_string()));
    }
}
