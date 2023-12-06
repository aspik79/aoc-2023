use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1 (contents: &str) {
    let items : Vec<_> = contents.lines().collect::<Vec<_>>();
    let mut tuples : Vec<(u32, u32)> = Vec::<_>::new();
    
    let times = parse_numbers_after_first(items[0]);
    let distances = parse_numbers_after_first(items[1]);

    for i in 0..times.len() {
        tuples.push((times[i], distances[i]));
    }

    println!("{}", count_solutions_for_tuples(&tuples));
}

fn part2 (contents: &str) {
    let items : Vec<_> = contents.lines().collect::<Vec<_>>();
    let time : u64 = read_folded_numbers_after_first(items[0]);
    let distance : u64 = read_folded_numbers_after_first(items[1]);

    println!("{}", count_solutions(time, distance));
}

fn parse_numbers_after_first(s: &str) -> Vec<u32> {
    s.split_whitespace().collect::<Vec<_>>()[1..].iter().map(|t| t.parse().unwrap()).collect::<Vec<u32>>()
}

fn read_folded_numbers_after_first(s: &str) -> u64 {
    let (_, s1) = s.split_once(":").unwrap();
    read_folded_numbers(s1) 
}

fn read_folded_numbers(s: &str) -> u64 {
    s.split_whitespace().fold(String::new(), |a, b| a + b).parse().expect("No number")
}

fn count_solutions_for_tuples (items: &Vec<(u32, u32)>) -> u64 {
    let mut result : u64 = 1;

    for (t, d) in items {
        result *= count_solutions((*t).into(), (*d).into());
    }

    result
}

fn count_solutions(time: u64, max_distance: u64) -> u64 {
    let mut result = 0;

    for press in 1..time {
        let distance = (time - press) * press;
        if distance > max_distance {
            result += 1;
        }
    }

    //println!("{} ways to win ({}, {}).", result, time, max_distance);

    result
}

