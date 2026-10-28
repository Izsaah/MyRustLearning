use crate::Color::Hsv;

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("using your favorite_color, {color}, as the bg");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color")
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
    match x {
        1 => println!("0ne"),
        2 => println!("Too"),
        3 => println!("Tree"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matche, y = {y}"),
        _ => println!("Default case , x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y= {y}");
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let msg = Message::ChangeColor(Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h},saturation {s},value {v}")
        }
        _ => (),
    }
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't over write an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting value is {setting_value:?}");
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first},{third},{fifth}");
        }
    }

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first},{last}");
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("the number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("matched, n = {n}"),
        _ => println!("Default case , x = {x:?}"),
    }
    println!("at the end: x = {x:?},y={y}");

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id:{id}"),
        _ => (),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x},{y})");
}
struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Hello { id: i32 },
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
fn foo(_: i32, y: i32) {
    println!("this code only use the y parameter:{y}");
}
