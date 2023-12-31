mod summarizable;
use summarizable::Summarizable;

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
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summarizable for NewsArticle { }

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
