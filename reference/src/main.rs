fn main() {
    references();
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    make_copy(x);
    let s = 1;
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2.clone());

    println!("{},{},{}", s1, s2, s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{s2}' is {len}.");

    let s1 = String::from("hello");

    let len = calculate_length_with_ref(&s1);
    println!("the length of '{s1}'is {len}.");

    let mut s = String::from("Hello");
    change(&mut s);
    println!("{s}");

    let mut s = String::from("more hellow");
    {
        let r1 = &mut s;
        println!("{r1}");
    }
    let r2 = &mut s;
    println!("{r2}");

    let s = String::from("Hello world");
    let word = first_word(&s);

    println!("{}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    //data type here is String
    let my_string = String::from("hello world");

    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);

    let _my_string_literal = "hello world";

    //data type here is &str
    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    let _word = first_word(&my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    let _array = assert_eq!(slice, &[2, 3]);
    // println!("{}", array);
}
fn references() {
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{s}");

    //integer value to variable x to y
    let x = 5;
    let _y = x;

    //String
    let x = String::from("hello");
    let _y = x;

    // println!("{x}");

    let mut _s = String::from("hello");
    let s = String::from("ahoy");
    println!("{s}, world!");

    let x = String::from("hello");
    let y = x.clone();
    println!("{},{}", x, y);
}
fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}
fn takes_ownership(string: String) {
    println!("{string}");
}
fn make_copy(int: i32) {
    println!("{}", int);
}
fn take_and_gives_back(a_string: String) -> String {
    a_string
}
//cheess the problem with tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str("string");
}

fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
