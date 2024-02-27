pub fn test() {
    println!(" [ Box test ] ");

    let mut a = 5;
    let b = Box::new(a);
    println!("a == {}", a);
    println!("b := Box(a)");
    println!("b == {}", b);

    println!("a := 10");
    a = 10;
    println!("a == {}", a);
    println!("b == {}", b);

    println!("b := &a");
    let b = &a;
    println!("b == {}", b);
}