use std::fmt::{Debug, Display};

fn main() {
    let article = ArticleNews {
        title: String::from("Biography"),
        author: String::from("John Doe"),
        content: String::from("Hello world")
    };
    println!("{0}", article.resume());

    let tweet = Tweet {
        user_name: String::from("John Doe"),
        content: String::from("Hello world"),
        retweet: true
    };
    println!("{0}", tweet.resume());

    let default_resume = DefaultResume {};
    println!("{0}", default_resume.resume());

    notify(&default_resume);
    notify_multiple_trait(&tweet);

    let article = get_resumable();
    notify(&article);
}

// trait Resumable {
//     fn resume(&self) -> String;
// }

trait Resumable {
    fn resume(&self) -> String {
        String::from("See more...")
    }
}

#[derive(Debug)]
struct ArticleNews {
    title: String,
    author: String,
    content: String
}

#[derive(Debug)]
struct Tweet {
    pub user_name: String,
    pub content: String,
    pub retweet: bool
}

#[derive(Debug)]
struct DefaultResume {

}

impl Resumable for ArticleNews {
    fn resume(&self) -> String {
        format!("{0}, by {1} ({2})", self.title, self.author, self.content)
    }
}

impl Resumable for Tweet {
    fn resume(&self) -> String {
        format!("{0}, by {1} ({2})", self.content, self.user_name, self.retweet)
    }
}

impl Resumable for DefaultResume {

}

// fn x<T: Resumable>(item: &T) {
//     println!("Notification: {0}", item.resume());
// }

fn notify(item: &impl Resumable) {
    println!("Notification: {0}", item.resume());
}

fn notify_multiple_trait(item: &(impl Resumable + Debug)) {
    println!("{:#?}", item);
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    42
}

fn get_resumable() -> impl Resumable {
    ArticleNews {
        title: String::from("Biography"),
        author: String::from("John Doe"),
        content: String::from("Hello world")
    }
}

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
    fn show_compare(&self) {
        if self.x >= self.y {
            println!("Greater is x = {}", self.x);
        } else {
            println!("Greater is y = {}", self.y);
        }
    }
}
