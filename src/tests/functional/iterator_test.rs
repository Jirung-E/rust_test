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

    let shoes = vec![
        Shoe::new(10, "sneaker"),
        Shoe::new(13, "sandal"),
        Shoe::new(10, "boot"),
    ];

    let in_my_size = shoe_in_size(shoes, 10);

    println!("{:#?}", in_my_size);
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

impl Shoe {
    fn new(size: u32, style: &str) -> Shoe {
        Shoe {
            size,
            style: style.to_string()
        }
    }
}


fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}