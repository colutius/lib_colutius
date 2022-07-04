use std::time::{SystemTime, UNIX_EPOCH};
pub enum Type {
    Common,
    Unknown,
    Error,
    Join,
    Kick,
    Nick,
    Notice,
    Part,
    Ping,
    Quit,
}
pub enum Who {
    Server(String),
    Channel(String),
    User(String),
    Me(String),
}

pub struct Message {
    //Type
    msg_type: Type,
    //Who
    on_server: String,
    from: Who,
    to: Who,
    //Main
    raw_msg: String,
    main_msg: String,
    //Additional
    time: u64,
    my_nick: String,
}
impl Message {
    pub fn new() -> Message {
        //获取时间戳
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("Time Error"),
        };
        //初始化消息
        Message {
            msg_type: Type::Unknown,
            on_server: "".to_string(),
            raw_msg: "".to_string(),
            main_msg: "".to_string(),
            from: Who::Server("".to_string()),
            to: Who::User("".to_string()),
            time,
            my_nick: "".to_string(),
        }
    }

    pub fn msg_type(mut self, msg_type: Type) -> Message {
        self.msg_type = msg_type;
        self
    }
    pub fn on_server(mut self, on_server: String) -> Message {
        self.on_server = on_server;
        self
    }
    pub fn raw_msg(mut self, raw_msg: String) -> Message {
        self.raw_msg = raw_msg;
        self
    }
    pub fn main_msg(mut self, main_msg: String) -> Message {
        self.main_msg = main_msg;
        self
    }
    pub fn from(mut self, from: Who) -> Message {
        self.from = from;
        self
    }
    pub fn to(mut self, to: Who) -> Message {
        self.to = to;
        self
    }
    pub fn my_nick(mut self, my_nick: String) -> Message {
        self.my_nick = my_nick;
        self
    }
    pub fn show(&self) {
        println!("on_server\t{}", self.on_server);
        println!("raw_msg:\t{}", self.raw_msg);
        println!("main_msg\t{}", self.main_msg);
        println!("time\t{}", self.time);
    }

    pub fn parse(mut self) -> Message {
        let raw_msg = self.raw_msg.clone();
        let on_server = self.on_server.clone();
        let my_nick = self.my_nick.clone();
        for (n, msg) in raw_msg.split(" ").enumerate() {
            match n {
                0 => {
                    if msg == "PING" {
                        self = self.msg_type(Type::Ping);
                        self = self.from(Who::Server(on_server));
                        self = self.to(Who::Me(my_nick));
                        self = self.main_msg(raw_msg.to_string().replacen("PING", "PONG", 1));
                        break;
                    } else {
                        self = self.parse_from(msg.to_string());
                    }
                }
                1 => {
                    self = self.parse_type(msg.to_string());
                }
                2 => {
                    self = self.parse_to(msg.to_string());
                }
                3 => {
                    self = self.main_msg(raw_msg[3..].to_string());
                    self.main_msg.remove(0);
                }
                _ => (),
            }
        }
        self
    }
    fn parse_to(mut self, msg: String) -> Message {
        if msg.chars().nth(0).unwrap() == '#' {
            self.to = Who::Channel(msg);
        } else {
            if msg == self.my_nick {
                self.to = Who::Me(msg);
            } else {
                self.to = Who::User(msg);
            }
        }
        self
    }
    fn parse_type(mut self, msg: String) -> Message {
        if msg == "PRIVMSG" {
            self.msg_type = Type::Common;
            return self;
        }
        if msg == "JOIN" {
            self.msg_type = Type::Join;
            return self;
        } else {
            self
        }
    }
    fn parse_from(mut self, msg: String) -> Message {
        for (n, ch) in msg.chars().enumerate() {
            if ch == '@' {
                self.from = Who::User(msg[0..n].to_string());
                return self;
            }
        }
        self.from = Who::Server(self.on_server.clone());
        self
    }
}
