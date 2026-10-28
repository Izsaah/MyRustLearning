static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
        dangerous();
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // println!("Absolute value of -3 according to C: {}", abs(-3));
    println!("{HELLO_WORLD}");
    unsafe {
        add_to_count(3);
        println!("COUNTER:{}", *(&raw const COUNTER));
    }
}
unsafe fn dangerous() {}

// unsafe extern "C" {
//     safe fn abs(input: i32) -> i32;
// }

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait Foo {}
unsafe impl Foo for i32 {}
