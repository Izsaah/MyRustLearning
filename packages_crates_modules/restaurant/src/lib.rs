use std::fmt;
use std::io as IoResult;
mod front_of_house;
use crate::front_of_house::hosting;

pub use crate::front_of_house::hosting as pub_hosting;
use crate::front_of_house::hosting::add_to_waitlist;
fn delivery_order() {}
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::delivery_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("Blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
    hosting::add_to_waitlist();
    add_to_waitlist();
    pub_hosting::add_to_waitlist();
}
fn func1() -> fmt::Result {
    Ok(())
}
// fn func2() -> IoResult<()> {
//     Ok(())
// }
