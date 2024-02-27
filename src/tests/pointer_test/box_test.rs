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


    println!("\n\n");


    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
