fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4, 5];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the thirs element is {third}"),
        _ => println!("there is no third element."),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let v = vec![100, 23, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("this is a string")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "xin chào ";
    let s = &hello[0..8];
    println!("{s}");

    for c in "chào".chars() {
        println!("{c}");
    }
    for c in "chào".bytes() {
        println!("{c}");
    }
} //vector goes out of scope and is freed here

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
