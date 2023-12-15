pub fn test() {
    println!("[ownership test]");

    let s = String::from("hello");
    let s1 = s;     // s moved to s1(s is no longer valid)
    let s2 = s1.clone();    // s1 is copied to s2(s1 is still valid)
    takes_ownership(s1);
    // println!("{}", s1); // error: value borrowed here after move
    println!("{}", s2);
    let s2 = takes_ownership_and_return_new(s2);    // s2 is moved to takes_ownership_and_return_new, and then moved back to s2
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2);  // s2 is moved to takes_and_gives_back, and then moved back to s3
    println!("{}\n", s3);

    let x = 5;
    let y = x;      // x is copied to y(x is still valid)
    makes_copy(x);
    makes_copy(y);
    println!("x: {}, y: {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_and_return_new(some_string: String) -> String {
    let some_string = some_string + " world";
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("copy: {}", some_integer);
}
