use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("guess a number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input your guest!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
