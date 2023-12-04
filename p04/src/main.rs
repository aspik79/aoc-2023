use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let items = contents.lines().collect::<Vec<_>>();
    let part1 : u32 = items.iter().map(|l| evaluate_card(l)).sum();
    println!("{part1:?}");

    let mut acc: u32 = 0;
    let mut counts : HashMap<u32, u32> = HashMap::new();
    for item in items.iter() {
        let (intro, _) = item.split_once(": ").unwrap();
        let ns = intro.split_whitespace().collect::<Vec<_>>()[1];
        
        let n : u32 = ns.parse().unwrap();
        let w = count_winners(item);
        let mut multiplier = 1;
        if counts.contains_key(&n) {
            multiplier += counts.get(&n).unwrap();
        }

        for i in 1..w+1 {
            match counts.get(&(n + i)) {
                Some(c) => counts.insert(n + i, c + multiplier),
                None => counts.insert(n + i, multiplier)
            };
        }

        //println!("adding {} of card {}", multiplier, n);

        acc += multiplier;
    }

    println!("{acc}");
}

fn evaluate_card(line: &str) -> u32 {

    let mut result = 0;

    let win_count = count_winners(line);
    if win_count > 0 {
        result = u32::pow(2, win_count - 1);
    }

    result
}

fn count_winners(line: &str) -> u32 {
    let (_, content) = line.split_once(": ").unwrap();
    let parts : Vec<&str> = content.split(" | ").collect();

    let winners : HashSet<&str> = parts[0].split_whitespace().collect::<HashSet<_>>();
    let candidates : HashSet<&str> = parts[1].split_whitespace().collect::<HashSet<_>>();

    let win_count : u32 = winners.intersection(&candidates).count().try_into().unwrap();
    win_count
}
