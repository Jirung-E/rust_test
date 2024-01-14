pub fn test() {
    println!(" [ String test ] ");

    let mut s = String::new();
    s.push_str("hello");
    s.push('!');
    println!("s: {}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + ", " + &s2 + "!";
    println!("s1 + \", \" + &s2 + \"!\": {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("s4: {}", s4);

    for c in s4.chars() {
        print!("{} ", c);
    }
    println!();
    for c in s4.bytes() {
        print!("{} ", c);
    }
    println!();

    for c in "안녕".chars() {
        print!("{} ", c);
    }
    println!();
    for c in "안녕".bytes() {
        print!("{} ", c);
    }
    println!();

    let s = "가나다라마바사아 안녕하세요 러스트";
    println!("s: {}", s);
    println!("s[0..3]: {}", &s[0..3]);
    println!("s[25..28]: {}", &s[25..28]);
}