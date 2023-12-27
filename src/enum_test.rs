pub fn test() {
    println!("[enum test]");
    test_message();
    println!();
    test_option();
}

fn test_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    println!("some_number: {}", some_number.unwrap());
    println!("some_string: {}", some_string.unwrap());
    let n = 10;
    println!("n + some_number: {}", n + some_number.unwrap());
}

fn test_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move { x: 10, y: 20 };
    m.call();
    let m = Message::ChangeColor(255, 255, 255);
    m.call();
    let m = Message::Quit;
    m.call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b)
        }
    }
}
