fn main() {
    println!("함수 선언을 아래쪽에 해도 된다.");

    let mut x = 10;
    let y = 2;

    println!("{} + {} = {}", x, y, test_func(10, 2));

    x = 10;
    test_match(x);

    test_for();

    test_if_else();
}

fn test_match(x: i32) {
    println!("test_match({})", x);
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("something else"),
    }
}

fn test_func(x: i32, y: i32) -> i32 {
    println!("x = {}, y = {}", x, y);
    return x + y;
}

fn test_for() {
    let mut x = 5;
    println!("x의 초기값: {}", x);
    x = 3;
    println!("x의 변경된 값: {}", x);
    println!("같은 이름의 변수로 for루프를 돈다.");
    for x in 0..10 {
        println!("{}", x);
    }
    println!("x: {} (x의 값이 바뀌지 않는다.)", x);
}

fn test_if_else() {
    let condition = true;
    println!("condition: {}", condition);
    println!("if 안에는 bool타입만 들어갈수 있다.");
    if condition {
        println!("in if block");
    }
    else {
        println!("in else block");
    }

    println!("변수 선언시에도 if-else를 사용할수 있다.");
    let number = if condition {
        5
    } else {
        6
    };
    println!("number: {}", number);

    println!("여러번 할수도 있다.");
    println!("같은 변수 이름을 사용해도 문제 없다.");
    let number = if number < 5 {
        1
    } else if number > 5 {
        10
    } else {
        0
    };
    println!("number: {}", number);
}