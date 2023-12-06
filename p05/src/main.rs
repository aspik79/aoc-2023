use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let blocks = contents.split("\r\n\r\n").collect::<Vec<_>>();
    println!("{}", blocks.len());

    let chain = &blocks[1..8];

    let mut maps : Vec<Vec<Vec<u64>>> = Vec::<_>::new();
    for block_index in 0..chain.len() {
        let block = chain[block_index];
        let block_content : Vec<_> = block.lines().collect();

        //let intro = &block_content[0];
        let data = &block_content[1..];

        let mut map : Vec<Vec<u64>> = Vec::<_>::new();
        for line in data {
            let tokens = line.split_whitespace().map(|item| item.parse().unwrap()).collect();
            map.push(tokens);
        }

        maps.push(map);
    }

    let seed_line = blocks[0];
    let p1 = part1(&maps, seed_line);
    println!("{}", p1);

    let p2 = part2(&maps, seed_line);
    println!("{}", p2);

}

fn part1(maps: &Vec<Vec<Vec<u64>>>, seed_line: &str) -> u64 {

    let seeds : Vec<u64> = seed_line.split(": ").collect::<Vec<_>>()[1].split_whitespace().map(|item| item.parse().unwrap()).collect();
    dbg!(&seeds);

    let mut acc: u64 = 99999999999;
    for seed in seeds {
        let mut current = seed;
        for map in maps.iter() {
            for line in map.iter() {
                let s = line[1];
                let l = line[2];
                if current >= s && current < s + l {
                    let offset = current - s;
                    current = line[0] + offset;

                    //println!("mapped to {}", current);
                    break;
                }
                else {
                    //println!("{} is not bewtween {} and {}", current, s, s + l);
                }
            }
        }

        if current < acc {
            println!("Min Loc reduced to {}", current);
            acc = current;
        }
    }

    acc
}

fn part2(maps: &Vec<Vec<Vec<u64>>>, seed_line: &str) -> u64 {

    let seed_range_tokens : Vec<u64> = seed_line.split(": ").collect::<Vec<_>>()[1].split_whitespace().map(|item| item.parse().unwrap()).collect();
    let range_count = seed_range_tokens.len() / 2;

    let mut next_tuples : Vec<(u64, u64)> = Vec::<_>::new();
    let mut current_tuples : Vec<(u64, u64)> = Vec::<_>::new();

    for sri in 0..range_count {
        current_tuples.push((seed_range_tokens[2 * sri], seed_range_tokens[2 * sri + 1]));
    }

    dbg!(&current_tuples);


    let acc = handle_next_map(&maps, &current_tuples);
    acc
}

fn handle_next_map(maps: &[Vec<Vec<u64>>], tuples: &Vec<(u64, u64)>) -> u64 {
    let acc;

    println!("{} maps left, {} tuples.", maps.len(), tuples.len());

    if maps.len() > 0 {
        let map = &maps[0];
        let mut next_tuples: Vec<(u64, u64)> = Vec::<_>::new();
        let mut uncovered: Vec<(u64, u64)> =  Vec::<_>::new();

        for (start, length) in tuples {
            for line in map.iter() {
                let map_start = line[1];
                let map_length = line[2];

                let intersect_start = cmp::max(map_start, *start);
                let intersect_end = cmp::min(map_start + map_length, start + length);
                if intersect_end > intersect_start {
                    let intersect_length = intersect_end - intersect_start;

                    let start_offset = intersect_start - map_start;
                    println!("new intersect tuple: {}, {}", intersect_start, intersect_length);
                    next_tuples.push((line[0] + start_offset, intersect_length));
                }
            }
        }

        acc = handle_next_map(&maps[1..], &next_tuples)
    }
    else
    {
        acc = *tuples.iter().map(|(x, _)| x).min().expect("no tuple for location left.");
    }

    return acc;
}
