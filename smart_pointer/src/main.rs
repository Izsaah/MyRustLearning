#[derive(Debug)]
enum List {
    // this look kinda dumb lol or maybe i am dumb
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// use crate::List::{Cons, Nil};
// fn main() {
//     let b = Box::new(5);
//     println!("b={b}");

//     let list = Cons(1, Cons(2, Nil));
// }
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    CHangeCOlor(i32, i32, i32),
}
use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{self, Rc, Weak};

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    // let List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    let c = CustomSmartPointer {
        data: String::from("Some data"),
    };

    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main ");

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c= {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope ={}", Rc::strong_count(&a));

    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;
    // println!("a after = {a:?}");
    // println!("b after = {b:?}");
    // println!("c after = {c:?}");

    // don't look at all the code with // i never bothered to come back and change their typed

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`! ", self.data);
    }
}
