//use std::cmp;
use std::env;
use std::fs;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let items = contents.lines();
}