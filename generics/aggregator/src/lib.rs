use std::fmt::Display;
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("(Read more {}...)", self.summarize_author())
//     }

//     fn summarize_author(&self) -> String {
//         todo!()
//     }
//     // fn summarize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }
// }

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
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
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}
