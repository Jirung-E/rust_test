use std::fmt::Display;


pub fn test() {
    println!(" [ lifetime test ] ");

    // longest_test_1();
    // longest_test_2();

    struct_test();

    let s: &'static str = "hello world!";

    test_all();
}


fn test_all() {
    println!("{}", longest_with_an_announcement("Hello", "Hi", "Longest?: "));
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}


#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}


fn struct_test() {
    let novel = String::from("Call me. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{i:?}");
}


fn longest_test_1() {
    let mut s1 = "hello";
    let mut s2 = "world";

    let lgst = longest(s1, s2);
    println!("longest: {}", lgst);

    s1 = "ffdasklj";
    s2 = "fjdkalsjgg";
    println!("longest: {}", lgst);

    let lgst = longest(s1, s2);
    println!("longest: {}", lgst);
}

fn longest_test_2() {
    let s1 = "hello";
    {
        let s2 = "rust";
        println!("longest: {}", longest(s1, s2));
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}