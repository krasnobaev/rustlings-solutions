// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    Move {x: i32, y: i32},
    EchoEnum,
    Red,
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }

    fn Echo(s: String) -> Self {
        Message::EchoEnum
    }

    fn ChangeColor(r: u8, g: u8, b: u8) -> Self {
        Message::Red
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
