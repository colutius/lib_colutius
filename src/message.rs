pub enum Who<T> {
    Server(T),
    People(T),
}

pub struct Message {
    raw: String,
    main_message: String,
    from: Who<String>,
    to: Who<String>,
}
impl Message {
    pub fn new() -> Message {
        Message {
            raw: "".to_string(),
            main_message: "".to_string(),
            from: Who::Server("".to_string()),
            to: Who::People("".to_string()),
        }
    }
    pub fn raw(mut self, raw: &str) -> Message {
        self.raw = raw.to_string();
        self
    }
    pub fn main_message(mut self, main_message: &str) -> Message {
        self.main_message = main_message.to_string();
        self
    }
    pub fn from(mut self, from: Who<String>) -> Message {
        self.from = from;
        self
    }
    pub fn to(mut self, to: Who<String>) -> Message {
        self.to = to;
        self
    }
    pub fn show(&self) {
        println!("raw:\t{}", self.raw);
        println!("main_message\t{}", self.main_message);
    }
}
