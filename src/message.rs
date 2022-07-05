#![allow(dead_code)]
#![allow(unused_variables)]
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub enum Type {
    Common,  //常规
    Unknown, //未知
    Error,   //错误
    Join,    //加入频道
    Kick,    //踢出频道
    Nick,    //更改昵称
    Notice,  //通知
    Part,    //退出频道
    Ping,    //PING
    Quit,    //退出服务器
}
#[derive(Debug)]
pub enum Who {
    Server(String),
    Channel(String),
    User(String),
    Me(String),
}
#[derive(Debug)]
pub struct Message {
    //Type
    msg_type: Type, //消息类型
    //Who
    on_server: String, //所在服务器
    from: Who,         //从哪来
    to: Who,           //到哪去
    //Main
    raw_msg: String,  //TCP传输的原始数据
    main_msg: String, //主体信息
    //Additional
    time: u64,       //发送时间，时间戳格式
    my_nick: String, //我的昵称
}
//setter
impl Message {
    ///构造
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
    ///设置消息类型
    pub fn msg_type(mut self, msg_type: Type) -> Message {
        self.msg_type = msg_type;
        self
    }
    ///设置所在服务器
    pub fn on_server(mut self, on_server: String) -> Message {
        self.on_server = on_server;
        self
    }
    ///设置原始消息
    pub fn raw_msg(mut self, raw_msg: String) -> Message {
        self.raw_msg = raw_msg;
        self
    }
    ///设置主体消息
    pub fn main_msg(mut self, main_msg: String) -> Message {
        self.main_msg = main_msg;
        self
    }
    ///设置from
    pub fn from(mut self, from: Who) -> Message {
        self.from = from;
        self
    }
    ///设置to
    pub fn to(mut self, to: Who) -> Message {
        self.to = to;
        self
    }
    ///设置我的昵称
    pub fn my_nick(mut self, my_nick: String) -> Message {
        self.my_nick = my_nick;
        self
    }
    ///展示消息结构，Debug用
    pub fn show(&self) {
        println!("[Type]");
        println!("msg_type\t{:?}", self.msg_type);
        println!("[Who]");
        println!("on_server\t{}", self.on_server);
        println!("from\t{:?}", self.from);
        println!("to\t{:?}", self.to);
        println!("[Main]");
        println!("raw_msg:\t{}", self.raw_msg);
        println!("main_msg\t{}", self.main_msg);
        println!("[Additional]");
        println!("time\t{}", self.time);
        println!("my_nick\t{}", self.my_nick);
    }
}
//parser
impl Message {
    ///解析原始数据
    pub fn parse(mut self) -> Message {
        let raw_msg = self.raw_msg.clone();
        let on_server = self.on_server.clone();
        let my_nick = self.my_nick.clone();
        self = self.msg_type(Type::Ping);
        for (n, msg) in raw_msg.split(" ").enumerate() {
            match n {
                0 => {
                    if msg == "PING" || msg == "ERROR" {
                        self = self.parse_special(msg.to_string());
                        break;
                    } else if raw_msg.split(" ").count() <= 3 {
                        self = self.main_msg(raw_msg.to_string());
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
                    self.main_msg.push_str(msg);
                    self.main_msg.push(' ');
                    self.main_msg.remove(0);
                    if n == raw_msg.split(" ").count() - 1 {
                        self.main_msg.pop();
                    }
                }
                _ => {
                    self.main_msg.push_str(msg);
                    self.main_msg.push(' ');
                    if n == raw_msg.split(" ").count() - 1 {
                        self.main_msg.pop();
                    }
                }
            }
        }
        self
    }
    ///解析消息接收方
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
    ///解析消息类型
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
    ///解析消息发送方
    fn parse_from(mut self, msg: String) -> Message {
        for (n, ch) in msg.chars().enumerate() {
            if ch == '!' {
                self.from = Who::User(msg[0..n].to_string());
                return self;
            }
        }
        self.from = Who::Server(self.on_server.clone());
        self
    }
    ///解析特殊消息
    fn parse_special(mut self, msg: String) -> Message {
        let raw_msg = self.raw_msg.clone();
        let on_server = self.on_server.clone();
        let my_nick = self.my_nick.clone();
        if msg == "PING" {
            self = self.msg_type(Type::Ping);
        } else if msg == "ERROR" {
            self = self.msg_type(Type::Error);
        } else {
            self = self.msg_type(Type::Unknown);
        }
        self = self.main_msg(raw_msg.to_string());
        self = self.from(Who::Server(on_server));
        self = self.to(Who::Me(my_nick));

        self
    }
}
//getter
impl Message {
    pub fn get_msg_type(&self) -> &Type {
        &self.msg_type
    }
    pub fn get_on_server(&self) -> &String {
        &self.on_server
    }
    pub fn get_from(&self) -> &Who {
        &self.from
    }
    pub fn get_to(&self) -> &Who {
        &self.to
    }
    pub fn get_raw_msg(&self) -> &String {
        &self.raw_msg
    }
    pub fn get_main_msg(&self) -> &String {
        &self.main_msg
    }
    pub fn get_time(&self) -> u64 {
        self.time
    }
}
