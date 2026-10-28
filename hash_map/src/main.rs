use std::{
    collections::{HashMap, btree_map::Values, hash_map},
    hash::Hash,
    io::{Read, stdin},
    ops::ControlFlow::Continue,
    os::windows::io,
    slice::RSplit,
    *,
};
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{},{}", field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split(" ") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    let mut v = vec![37, 8, 3, 46, 6, 38, 33, 38, 24, 28];

    v.sort();
    let mut map: HashMap<String, i32> = HashMap::new();
    let result = v[v.len() / 2 + 1];
    println!("{result}");

    for i in v {
        let count = map.entry(i.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    let string = "xin chào thế giới"; // a,i,u,e,o = +hay, everything else move the first consonant to last + ay
    let mut vec: Vec<String> = Vec::new();
    for word in string.split(" ") {
        if word.starts_with(&['a', 'i', 'u', 'e', 'o']) {
            let _new_word = vec.push(String::from(word) + "hay");
        } else {
            let mut tmp_chars: Vec<char> = word.chars().collect();
            let first = tmp_chars.remove(0);
            tmp_chars.push(first);
            let word: &String = &tmp_chars.into_iter().collect();
            let _new_word = vec.push(String::from(word) + "ay");
        }
    }
    let result = vec.join(" ");
    println!("{result}");
    // there still gonna be bug on the way but i think i'm done for now
    let mut _temp_emp: Vec<String> = Vec::new();
    let mut _temp_dep: Vec<String> = Vec::new();
    let mut result: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    let mut loop_vo = true;
    let mut loop_vi = true;
    let mut input = String::new();

    while loop_vo {
        // check for decision
        println!(
            "You can either add an employee to their department or you can list the company out via: a/l"
        );
        stdin().read_line(&mut input).expect("err");
        let decision = input.clone();
        input.clear();

        match decision.trim() {
            "a" => {
                println!("You can add an employee here");
                stdin().read_line(&mut input).expect("err");
                let employee = input.clone();
                input.clear();

                println!("You can add his department here");
                stdin().read_line(&mut input).expect("err");
                let department = input.clone();
                input.clear();

                _temp_emp.push(employee);
                _temp_dep.push(department);

                while loop_vi {
                    println!("do you wish to keep adding the employee to this department y/n");
                    stdin().read_line(&mut input).expect("err");
                    let decision = input.clone();
                    input.clear();
                    match decision.trim() {
                        "y" => {
                            println!("You can add an employee here");
                            stdin().read_line(&mut input).expect("err");
                            let employee = input.clone();
                            input.clear();
                            _temp_emp.push(employee);
                            _temp_emp.sort_by(|a, b| a.cmp(b));
                        }
                        "n" => {
                            println!("The employee have been save!");
                            result
                                .entry(_temp_dep.clone())
                                .insert_entry(_temp_emp.clone());

                            _temp_emp.clear();
                            _temp_dep.clear();
                            break;
                        }
                        _ => break,
                    }
                }
            }
            "l" => {
                println!(
                    "do you wish listing out the company or you can list out the department via: lc/ld"
                );
                stdin().read_line(&mut input).expect("err");
                let decision = input.clone();
                input.clear();
                match decision.trim() {
                    "lc" => {
                        let mut sorted: Vec<_> = result.iter().collect();
                        sorted.sort_by_key(|a| a.0);
                        for (key, value) in sorted.iter() {
                            println!("KEY, VALUE: {key:?} {value:?}");
                        }
                    }
                    "ld" => {
                        println!("Which department you wanna see: ?");
                        stdin().read_line(&mut input).expect("err");
                        let decision = input.clone();
                        input.clear();
                        let mut sorted: Vec<_> = result.iter().collect();
                        sorted.sort_by_key(|a| a.0);
                        for (key, value) in sorted.iter() {
                            if key.contains(&decision) {
                                println!("KEY, VALUE: {key:?} {value:?}");
                            }
                        }
                    }
                    _ => break,
                }
            }
            _ => break,
        }
    }

    // let mut vec: Vec<String, String> = Vec::new();
    // let mut map: HashMap<String, Vec<String>> = HashMap::new();
    // //if find add go to the next word and add it into the vector till find to then add the vector and the word next to it
    // let mut input = String::new();

    // while true {
    //     println!(
    //         "Do you want to add an employee or listing out the employee y for add /n for listing {input}"
    //     );
    //     io::stdin().read_line(&mut input).expect("err");
    //     match input.trim() {
    //         "y" => {
    //             println!("you can add an employee via command: Add + Name + to + Department ");
    //             stdin().read_line(&mut input).expect("err");
    //             let vector_input: Vec<&str> = input.split(' ').collect();
    //             vec.push(vector_input[1].into());
    //             map.entry(vector_input[3].into())
    //                 .or_insert(vec.clone().into());
    //             println!("this is map {map:?} and this is the vector {vec:?} ");
    //             input.clear();
    //         }
    //         "n" => {
    //             stdin().read_line(&mut input).expect("err");
    //         }
    //         _ => {
    //             println!("this command may happened with good intent or completely broken");
    //             break;
    //         }
    //     }
    // }
    // let mut map: HashMap<String, i32> = HashMap::new();
    // let mut count = 0;
    // for word in string.split(" ") {
    //     if word.starts_with(&['a', 'i', 'u', 'e', 'o']) {
    //         let _new_world = map.entry(String::from(word) + "hay").or_insert(count);
    //     } else {
    //         let mut tmp_chars: Vec<char> = word.chars().collect();
    //         let first = tmp_chars.remove(0);
    //         tmp_chars.push(first);
    //         let word: &String = &tmp_chars.into_iter().collect();
    //         let _new_world = map.entry(word.clone() + "ay").or_insert(count);
    //     }
    //     count += 1;
    // }
    // println!("{map:?}");
    // let mut v: Vec<(String, i32)> = map.into_iter().collect();
    // v.sort_by(|a, b| b.0.cmp(&a.0));
    // let result: String = v
    //     .iter()
    //     .map(|(text, num)| text.as_str())
    //     .collect::<Vec<&str>>()
    //     .join(" ");

    // println!("{result}");
}
