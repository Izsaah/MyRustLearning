use crate::garden::vegetable::Asparagus;

use std::io::{self, Write};
use std::*;
use std::{cmp::Ordering, collections::HashMap};
//jsyk this is bs
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let mut map = HashMap::new();
    map.insert(1, 2);
}
