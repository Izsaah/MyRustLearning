fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("myNameIs"),
        email: String::from("MyNameIs@a"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@a");

    let user2 = User {
        email: String::from("anoh@"),
        ..user1
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "the area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );

    let rectangle: Rectangle = Rectangle {
        width: 100,
        height: 50,
    };
    println!("the area is {}", area_rectangle(&rectangle));
    println!("the area is {rectangle:?}");

    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rectangle);

    let rectangle = Rectangle {
        width: 30,
        height: 100,
    };
    println!(
        "the area of the rectangle is {} square pixels",
        rectangle.area()
    );

    println!(
        "the rectanglge has a non zero width; it is {}",
        rectangle.width()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let rect4 = Rectangle::square(3);
    println!("the area {rect4:?}");
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
