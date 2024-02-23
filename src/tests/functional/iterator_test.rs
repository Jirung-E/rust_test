pub fn test() {
    println!(" [ iterator test ] ");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    println!("sum: {}", v1.iter().sum::<i32>());

    println!("map(+1): {:?}", v1.iter().map(|x| x+1).collect::<Vec<_>>());
}