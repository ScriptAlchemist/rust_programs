#[allow(dead_code)]

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl std::fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} wrote: {}", self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl std::fmt::Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} wrote: {}", self.username, self.content)
    }
}

pub fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("Display: {}\n", item);
    println!("Breaking news! {}\n", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("ScriptAlchemist"),
        content: String::from("Hope everyone is having a nice day!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Fliers win the Stanley Cup Championship!"),
        location: String::from("Philadelphia, PA, USA"),
        author: String::from("Justin"),
        content: String::from(
            "The Philadelphia Fliers once again are the best \
            hockey team in the NHL.",
        ),
    };
    notify(&article);
    notify(&tweet);
}














