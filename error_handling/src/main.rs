use core::error;
use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
    result,
};

fn main() {
    // this chapter presented with many panic call it is a good idea to just // it before go to the next section
    // panic!("crash and burn");
    // let v = vec![1];
    // v[2];
    // let file_result = File::open("hello.txt");

    // let greeting_file = match file_result {
    //     Ok(file) => file,
    //     Err(err) => panic!("Problem opeing the file : {err:?}"),
    // };

    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating the file: {error:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };
    // let _greeting_file = File::open("hello.txt").unwrap();
    // let _greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
    //_read_username_from_file();
}
// fn _read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// fn _read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }
fn _read_username_from_file() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}
fn _last_char_of_first_line(tex: &str) -> Option<char> {
    tex.lines().next()?.chars().last()
}
