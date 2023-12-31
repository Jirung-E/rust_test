mod summarizable;
use summarizable::Summarizable;
use std::cmp::PartialOrd;

pub fn test() {
    println!(" [ trait test ] ");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("New article available! {}", article.summary());

    notify(article);

    println!("------------------------------------------");

    let numbers: Vec<u32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("numbers: {:?}", numbers);
    println!("largest: {}", largest(&numbers));
}

fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &e in list.iter() {
        if e > largest {
            largest = e;
        }
    }
    largest
}

fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("#{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.author_summary(), self.content)
    }
}
