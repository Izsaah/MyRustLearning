use aggregator::{NewsArticle, SocialPost, Summary};
use core::num;
use std::{
    clone,
    fmt::{Debug, Display},
    io::Lines,
    result,
};
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest_name = &number_list[0];

    for number in &number_list {
        if number > largest_name {
            largest_name = number;
        }
    }
    println!("the largest number is {largest_name}");

    let number_list = vec![10, 5, 8, 9, 10];
    let resutl = largest_i32(&number_list);
    println!("The largest number is {resutl}");

    let number_list = vec![1, 500, 800, 92, 10];
    let resutl = largest_i32(&number_list);
    println!("The largest number is {resutl}");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&char_list);
    println!("the largest char is {result}");

    let result = largest(&number_list);
    println!("the largest char is {result}");

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 5.0, y: 10.0 };
    let _integer_float = PointTf { x: 5, y: 10.0 };

    println!("integer.x : {}", _integer._x());

    let p1 = PointTf { x: 5, y: 10.4 };
    let p2 = PointTf { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {},p3.y={}", p3.x, p3.y);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // println!("New article avaible {}", article.summarize());
}
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for char in list {
        if char > largest {
            largest = char;
        }
    }
    largest
}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
struct Point<T> {
    x: T,
    y: T,
}
struct PointTf<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointTf<T, U> {
    fn mixup<X1, Y1>(self, other: PointTf<X1, Y1>) -> PointTf<T, Y1> {
        PointTf {
            x: self.x,
            y: other.y,
        }
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking new! {}", item.summarize());
// }

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     1
// }
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you problably already know, people"),
        reply: false,
        repost: false,
    }
}
