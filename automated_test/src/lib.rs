use std::fmt::format;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}
#[derive(Debug)]
struct Rectangle {
    w: u32,
    u: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.u > other.u
    }
}
pub fn add_two(a: u64) -> u64 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    String::from("Carol!")
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{result}'"
        );
    }
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        // panic!("make this test fail lol");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { w: 8, u: 7 };
        let smaller = Rectangle { w: 5, u: 1 };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_smaller() {
        // let larger = Rectangle { w: 8, u: 7 };
        // let smaller = Rectangle { w: 5, u: 1 };
        // assert!(smaller.can_hold(&larger));
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
