use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (directions_string, network_string) = contents.split_once("\r\n\r\n").unwrap();
    let mut network : HashMap<&str, (&str, &str)>= HashMap::new();
    for item in network_string.lines() {
        let (source, remain) = item.split_once(" ").unwrap();
        let left = &remain[3..6];
        let right = &remain[8..11];
        network.insert(source, (left, right));
    }

    //dbg!(&network);

    let mut count = 0;
    /*
    let mut location = "AAA";
    while location != "ZZZ" {
        for direction in directions_string.chars() {
            let (l, r) = network[location];
            location = match direction {
                'L' => l,
                'R' => r,
                _ => ""
            };
            count += 1;

            if location == "ZZZ" {
                break;
            }
        }
    }

    println!("{}, {}", location, count);
    */

    let mut locations : Vec<_> = network.keys().filter(|k| &k[2..3] == "A").map(|s|*s).collect();
    count = 0;
    while true {
        for direction in directions_string.chars() {
            for loc_index in 0..locations.len() {
                let location = locations[loc_index];
                let (l, r) = network[location];
                locations[loc_index] = match direction {
                    'L' => &l,
                    'R' => &r,
                    _ => &""
                };
            }

            count += 1;

            if count % 1000000 == 0 {
                println!("{}", count);
            }

            if is_terminal(&locations, count) {
                break;
            }
        }
    }
    
    dbg!(&locations);
    println!("{}", count);

    // solution after period inspection: 13385272668829
}

fn is_terminal(locs: &Vec<&str>, c: u32) -> bool {
    for item_index in 0..locs.len() {
        println!("{}", &locs[item_index]);
        if &locs[item_index][2..3] == "Z" {
            println!("{} reached {} at {}", item_index, &locs[item_index], c);
        }
    }
    println!("");

    is_terminal1(locs)
}

fn is_terminal1(locs: &Vec<&str>) -> bool {
    (*locs).iter().fold(true, |acc: bool, l: &&str| acc && &l[2..3] == "Z")
}
