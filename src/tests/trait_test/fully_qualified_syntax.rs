pub fn test() {
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
