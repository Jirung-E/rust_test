pub fn test() {
    println!("[enum test]");
    test_message();
    println!();
    test_option();
    println!();
    test_match();
}

fn test_match() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let number: u8 = 3;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => {}
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
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

    let some_number = plus_one(some_number);
    println!("some_number + 1: {:?}", some_number.unwrap());

    let none = plus_one(None);
    println!("none + 1: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1)
    }
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
