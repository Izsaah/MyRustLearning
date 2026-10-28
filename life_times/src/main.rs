use std::result;

fn main() {
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = "xyz";
    //     let result = longest(string1.as_str(), string2);
    // }
    // println!("The longest string is {result}");
    let novel = String::from("call me Ishmael. Some year ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
// fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {announcement}");
        self.part
    }
}
// this chapter should be look into since it hard to understand without actual implementation
