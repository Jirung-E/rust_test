mod gui;
mod blog;


pub fn test() {
    println!(" [ oop test ] ");

    blog_test();
}


fn polymorphism() {
    use gui::{Button, SelectBox, Screen};

    println!(" [ polymorphism ] ");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    screen.run();
}

fn blog_test() {
    use blog::Post;

    println!(" [ design pattern test ] ");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}