pub fn test() {
    println!(" [ pattern and matching ] ");

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");
        }
    }

    let ((feet, inches), Point { x, y, z: _ }) = ((3, 10), Point { x: 3, y: -10, z: 2 });
    println!("{feet} {inches} {x} {y}");

    foo(3, 4);

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    println!();

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => ()
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),      // priority: (4 | 5 | 6) if y
        _ => println!("no"),
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {id_variable}");
        }
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message2::Hello { id } => println!("Found some other id: {id}")
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color)
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

enum Message2 {
    Hello { id: i32 },
}