use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut iter = contents.lines();

    let mut amounts = HashMap::new();
    amounts.insert("red", 12);
    amounts.insert("green", 13);
    amounts.insert("blue", 14);

    let mut acc : i32 = 0;
    for item in iter {
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let main_parts: Vec<&str> = item.split(": ").collect();
        let intro_parts: Vec<&str> = main_parts[0].split_whitespace().collect();
        let game_no : i32 = intro_parts[1].parse().unwrap();

        let game_parts: Vec<&str> = main_parts[1].split("; ").collect();
        let mut feasible = true;
        for draw in game_parts {
            let draw_parts: Vec<&str> = draw.split(", ").collect();
            for draw_part in draw_parts {
                let draw_items: Vec<&str> = draw_part.split_whitespace().collect();
                let n: i32 = draw_items[0].parse().unwrap();
                let c = draw_items[1];
                let limit = amounts.get(&c).unwrap();

                feasible &= limit >= &n;
            }
        }

        if feasible {
            acc += game_no;
        }
    }

    println!("{}", acc);

    acc = 0;
    iter = contents.lines();

    for item in iter {
        let main_parts: Vec<&str> = item.split(": ").collect();
        let game_parts: Vec<&str> = main_parts[1].split("; ").collect();
        let mut min_amounts = HashMap::new();

        for draw in game_parts {
            let draw_parts: Vec<&str> = draw.split(", ").collect();
            for draw_part in draw_parts {
                let draw_items: Vec<&str> = draw_part.split_whitespace().collect();
                let n: i32 = draw_items[0].parse().unwrap();
                let c = draw_items[1];

                if !min_amounts.contains_key(c) || min_amounts.get(c).unwrap() < &n {
                    min_amounts.insert(c, n);
                } 
            }
        }

        let mut power = 1;
        for (c, &n) in min_amounts.iter() {
            power *= n;
        }

        acc += power;
        println!("acc {}", acc);
    }
}