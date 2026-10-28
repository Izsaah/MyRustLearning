fn main() {
    let answer = do_twice(add_one, 5);

    println!("the answer is :{answer}");

    let handlers = vec![return_closure(), returns_initialized_closure(132)];
    for handler in handlers {
        let output = handler(5);
        println! {"{output}"};
    }
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
