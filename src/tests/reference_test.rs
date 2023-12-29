pub fn test() {
    println!("[reference test]");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {}, r2: {}", r1, r2);

    // change(&mut r1);     // 이런식의 변환은 허용되지 않음
    println!("{}", r1);
    let len = calculate_length(r1);
    println!("The length of '{}' is {}.", r1, len);

    // let r3 = &mut s2;
    // let r4 = &mut s2;
    // println!("{}, {}", r3, r4);  // error: cannot borrow mutable more than once at a time
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
