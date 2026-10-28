use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirt: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirt {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirt: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} get {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} get {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrow_mut = || list.push(7);
    borrow_mut();
    println!("After calling closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before we calling the closure: {list:?}");
    only_borrows();
    println!("After we calling the closure:{list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
