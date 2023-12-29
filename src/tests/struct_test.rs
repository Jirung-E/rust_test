pub fn test() {
    println!("[struct test]");
    test_user();
    println!();
    test_menu();
    println!();
    test_rectangle();
}

fn test_user() {
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
    user2.username = String::from("UseR2_");
    user2.email = String::from("user2email@example.com");
    user2.sign_in_count = 10;
    user2.active = false;
    println!("user2 info: {:#?}", user2);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn test_menu() {
    let menu = Menu {
        name: String::from("test menu"),
        price: 1000
    };
    menu.show();

    let menus: [Menu; 5] = [
        Menu {
            name: String::from("chicken"),
            price: 12000
        },
        Menu {
            name: String::from("pizza"),
            price: 15000
        },
        Menu {
            name: String::from("burger"),
            price: 8000
        },
        Menu::new(String::from("orange juice"), 3000),
        Menu::new(String::from("coke"), 2000),
    ];
    for menu in menus {
        menu.show();
    }
}

#[derive(Debug)]
struct Menu {
    name: String,
    price: u32
}

impl Menu {
    fn new(name: String, price: u32) -> Menu {
        Menu {
            name,
            price
        }
    }

    fn show(&self) {
        println!("{}({})", self.name, self.price);
    }
}


fn test_rectangle() {
    let rect1 = Rectangle { length: 50, width: 30 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("square: {:#?}", square);

    println!("square is square? {}", square.is_square());
    println!("rect1 is square? {}", rect1.is_square());

    println!("square perimeter: {}", square.perimeter());
    println!("rect1 perimeter: {}", rect1.perimeter());
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn is_square(&self) -> bool {
        self.length == self.width
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.length) * 2
    }
}
