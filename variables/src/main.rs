fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const LABEL_NAME: &str = "this is a label";
    variable_and_mutabilityf();
    variable_and_mutabilityw();
    shadowing();
    datatype();
    parameter(THREE_HOURS_IN_SECONDS, LABEL_NAME);
    statement();
    let x = return_values();
    println!("{}", x);
    control_flow(i32::try_from(THREE_HOURS_IN_SECONDS).unwrap());
    //using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);
    let max = 10;
    loop_till_max(max);
    disambiguating_loop_labels();
    while_till_max();
    for_loop();
    convert_c_to_f_viceversa(100.58, true);
    generate_fibonacci_number(0, 1, 500);
    print_chrismart_carl();
}
fn variable_and_mutabilityf() {
    let x = 5;
    println!("{}", x);
    //x = 6;
    println!("this function doesnt work over all");
}
fn variable_and_mutabilityw() {
    let mut x: i32 = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
}
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("{}", x);
    }
    print!("{}", x);
    //shadowing use
    let spaces = " ";
    let spaces = spaces.len();
    print!("{}", spaces);
    //wrong way to do it
    // let mut spaces = " ";
    // spaces = spaces.len();
}
fn datatype() {
    //rust need to know which type you refer to...
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);
    //float
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{},{}", x, y);
    //sum
    let sum = 5 + 10;
    println!("{}", sum);
    //subtract
    let difference = 95.5 - 4.2;
    println!("{}", difference);
    //mutiplier
    let product = 4 * 31;
    println!("{}", product);
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("{},{}", quotient, truncated);
    //remainder
    let remainder = 43 % 5;
    println!("{}", remainder);
    //bool
    let t = true;
    //this required to explicit state
    let f: bool = false;
    println!("{},{}", t, f);
    //char "noted chance are you will rarely use this"
    let c = 'c';
    let z: char = 'ℤ';
    let heart_eye_cat = '😻';
    println!("{},{},{}", c, z, heart_eye_cat);
    // tuple(matrix) and array
    //tuple
    let x: (i32, f64, u8) = (500, 2.0, 1);

    //access the tuple
    let five_hundred = x.0;
    let two_point_o = x.1;
    let one = x.2;
    println!("{},{},{}", five_hundred, two_point_o, one);
    //array
    //noted should just use vector as it can shrink or expand depend on your needed
    let a = [1, 2, 3, 4, 5];

    let last = a[a.len() - 1];
    println!("{}", last);
}
//functions
fn parameter(x: u32, label: &str) {
    println!("the value of this constant {}, {}", x, label);
}
//statement and expression
fn statement() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of this is {}", y);
}
//functions return value
fn return_values() -> i32 {
    8
}
//control flow
fn control_flow(ntmp: i32) {
    let number: i32 = ntmp;
    if number < 5 {
        println!("true, the value is smaller than 5");
    } else {
        println!("false, the value is 5 or bigger than 5 ");
    }
}
//loop
fn loop_till_max(mut counter: i32) {
    let result = loop {
        if counter == i32::MAX {
            break (counter as i64) * 2;
        }
        counter += 1;
    };
    println!("result: {}", result);
    println!("last count: {}", counter);
}
fn disambiguating_loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("countr = {count}");
        let mut remaining = 10;
        loop {
            println!("remain = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn while_till_max() {
    let mut number = 3;
    while number != 3 {
        println!("{number}");
        number -= 1;
    }
    println!("MAX!!!");
}
fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for tmp in a {
        println!("the value :{tmp}");
    }
}
//true mean convert from c to f vice versa
fn convert_c_to_f_viceversa(mut temp: f32, temp_rule: bool) {
    if temp_rule {
        temp = (temp * 1.8) + 32.0;
        println!("f:{temp}");
    } else {
        temp = (temp - 32.0) * 5.0 / 9.0;
        println!("f:{temp}");
    }
}

fn generate_fibonacci_number(mut first_n: i32, mut sec_n: i32, condition: i32) {
    while first_n < condition {
        println!("{first_n}");
        let tmp_num = first_n + sec_n;
        first_n = sec_n;
        sec_n = tmp_num;
    }
}
fn print_chrismart_carl() {
    let a: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut count = 0;
    for _day in &a {
        count += 1;

        println!("On the {} day of Christmas", days[count - 1]);
        println!("My true love gave to me");
        for i in (0..count).rev() {
            if i == 0 && count > 1 {
                println!("and {}", a[i]);
            } else {
                println!("{}", a[i]);
            }
        }
    }
}
fn _rust_refer(s: &String) -> String {
    s.to_string()
}
