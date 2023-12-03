use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut iter = contents.split("\n");

    let mut acc : u32 = 0;
    for item in iter {
        let mut start : u32 = 0;
        let mut end : u32 = 0;
        for c in item.chars() {
            if c.is_numeric() {
                let d = c.to_digit(10).unwrap();
                if start <= 0 {
                    start = d;
                }

                end = d;
            }
        }

        let v = 10 * start + end;
        acc += v;
        println!("{}, {}", v, item);
    }

    println!("{}", acc);
    
    // Part 2
    acc = 0;
    iter = contents.split("\n");
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    for item in iter {
        let mut start : u32 = 0;
        let mut end : u32 = 0;

        for i in 0..item.len() {
            let part = &item[i..];

            let c = part.chars().nth(0).unwrap();
           
            if c.is_numeric() {
                let d = c.to_digit(10).unwrap();
                if start <= 0 {
                    start = d;
                }

                end = d;
            }
            else {
                for ni in 0..numbers.len() {
                    let ns = numbers[ni];

                    if part.len() >= ns.len() && &part[0..ns.len()] == ns {
                        if start <= 0 {
                            start = ni as u32;
                        }
        
                        end = ni as u32;
                    }
                }
            }
        }

        let v = 10 * start + end;
        acc += v;
        println!("{}, {}", v, item);
    }

    println!("{}", acc);
}
