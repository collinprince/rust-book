pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more by {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub metadata: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted: {}", self.summarize_author(), self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// a function which takes in a type that implements Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// a function which also takes in a type that implements Summary trait
// the original notify function is actually syntactic sugar for this
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news v2! {}", item.summarize());
}

// if we wanted two parameters to be of the same type, we would have to use the above Trait bound
// syntax
pub fn notify_double<T: Summary>(item1: &T, item2: &T) {
    println!("First piece of breaking news! {}", item1.summarize());
    println!("Second piece of breaking news! {}", item2.summarize());
}

pub fn notify_double_nonbound(item1: &impl Summary, item2: &impl Summary) {
    println!("First piece of breaking news! {}", item1.summarize());
    println!("Second piece of breaking news! {}", item2.summarize());
}

// we can also specify that we want a type that implements multiple traits with the + syntax
pub fn notify_with_display(item: &(impl Summary + Display)) {}

pub fn notify_with_bound_display<T: Summary + Display>(item: &T) {}

pub fn trait_bound_with_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

use std::fm::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// use impl Trait syntax for return type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        metadata: String::from("los angeles, ca"),
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("the_fresh_prince"),
        content: String::from("hello mutuals"),
        metadata: String::from("los angeles, ca"),
    };

    let news = NewsArticle {
        author: String::from("Collin Prince"),
        location: String::from("Los Angeles"),
        headline: String::from("Nice"),
        content: String::from("top of the morning"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", news.summarize());
    notify(&tweet);
    notify_v2(&tweet);

    // this will work as both news & tweet implement Summary even though they are different types
    notify_double_nonbound(&tweet, &news);

    // will not work, has to both be same type
    // notify_double(&tweet, &news);

    let tweet2 = Tweet {
        username: String::from("the_fresh_prince2"),
        content: String::from("goodbye mutuals"),
        metadata: String::from("los angeles, ca"),
    };

    // this will work as both args of type Tweet
    notify_double(&tweet, &tweet2);
}
