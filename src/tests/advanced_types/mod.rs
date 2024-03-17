use std::any::type_name;

pub fn test() {
    println!(" [ advanced types test ] ");

    alias_test();
    // never_type_test();
    // Sized, ?Sized
}

fn alias_test() {
    println!(" [ alias test ] ");

    type Test = i32;
    let x: i32 = 5;
    let y: Test = 5;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("type_name::<Test>(): {}", type_name::<Test>());
    println!("type_name::<i32>(): {}", type_name::<i32>());
    println!("x + y: {}", x + y);
    println!("x == y: {}", x == y);

    println!("Result<i32>: {}", type_name::<std::io::Result<i32>>());
}