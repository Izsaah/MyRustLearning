use std::fmt::format;

use crate::{
    Coin::Q,
    Option::{None, Some},
    UsState::{Alabama, Alaska},
};

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Q(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the maximun is configured to be {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("the maximun is configured to be {max}");
    }

    let mut _count = 0;

    let coin = Q(Alaska);
    match &coin {
        Coin::Q(state) => println!("state quarter from {state:?}"),
        _ => _count += 1,
    }
    // why is the ref here is broken i don't get it
    let mut count = 0;
    if let Coin::Q(state) = &coin {
        println!("state quarter from {state:?}");
    }
    //this is bull shit refering lol
    describe_state_quarter(coin);
    println!();
}
fn _route(_ip_kind: IpAddrKind) {
    _route(IpAddrKind::V4(String::from("127.0.0.1")));
    _route(IpAddrKind::V6(String::from("::1")));
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind {
    V4(String),
    V6(String),
}
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //this is a void call
    }
}

enum Option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    P,
    N,
    D,
    Q(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::P => {
            println!("Lucky Penny!");
            1
        }
        Coin::N => 5,
        Coin::D => 10,
        Coin::Q(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(numspace: u8) {}
fn reroll() {}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1819,
        }
    }
}
// bruh the book didn't show me the way to format this lol
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Q(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for usa!"))
        } else {
            Some(format!("{state:?} is relatively new "))
        }
    } else {
        None
    }
}
