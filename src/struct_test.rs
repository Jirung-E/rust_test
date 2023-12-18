pub fn test() {
    let user1 = User {
        email: String::from("someone@exmaple.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };
    println!("user1 info: {:#?}", user1);

    let mut user2 = User {
        username: String::from("user2"),
        email: String::from("user2email.example.com"),
        ..user1
    };
    println!("user2 info: {:#?}", user2);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
