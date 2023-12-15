fn main() {
    test_ownership();
}


fn test_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
