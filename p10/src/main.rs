use std::cmp;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let items = contents.lines();
    let start_candidates : Vec<_> = vec!['|', '-', 'L', 'J', '7', 'F'];
    let mut moves: HashMap<char, (u8, u8)> = HashMap::new();
    moves.insert('|', (1, 3));
    moves.insert('-', (0, 2));
    moves.insert('L', (0, 1));
    moves.insert('J', (1, 2));
    moves.insert('7', (2, 3));
    moves.insert('F', (0, 3));

    let mut matrix : Vec<_> = Vec::<Vec::<char>>::new();
    let mut start_loc: (i16, i16) = (0, 0);

    let mut ri = 0;
    for item in items {
        match item.find('S') {
            Some(ci) => start_loc = (ri, ci.try_into().unwrap()),
            _ => {}
        }
        matrix.push(item.chars().collect());
        ri += 1;
    }

    let (sri, sci) = start_loc;
    let mut max_path_len : u32 = 0;
    dbg!((&sri, &sci));

    let mut max_maze : HashSet::<(i16, i16)> = HashSet::new();
    for start_c in start_candidates {
        println!("Candidate: {}", start_c);
        matrix[sri as usize][sci as usize] = start_c;

        let mut maze : HashSet<_> = HashSet::<(i16, i16)>::new();

        let (d_start, _) = moves.get(&start_c).unwrap();
        let mut loc = start_loc;
        let mut dist = 0;
        let mut from_dir = (d_start + 2) % 4;
        while loc != start_loc || dist == 0 {
            maze.insert(loc);
            dist += 1;
            let (r, c) = loc;
            let ch : char = matrix[r as usize][c as usize];
            if ch == '.' {
                break;
            }

            let (d1, d2) = moves.get(&ch).unwrap();
    
            let next_dir = if (from_dir + 2) % 4 == *d1 {
                d2
            }
            else {
                d1
            };
    
            let (nr, nc) = step(loc, *next_dir);
            if nr >= 0 && nc >= 0 && &(nr as usize) < &matrix.len() && &(nc as usize) < &matrix[nr as usize].len() {
                loc = (nr, nc);
                from_dir = *next_dir;
            }
            else {
                println!("loc {},{} out of bounds", nr, nc);
            }

        }

        if loc == start_loc && (d_start + 2) % 4 == from_dir {
            println!("{}, last dir: {}, maze size: {}", dist / 2, from_dir, maze.len());
            max_path_len = cmp::max(max_path_len, dist);
            max_maze = maze;
        }
    }

    println!("{}", max_path_len / 2);
    println!("{}", max_maze.len());

    let mut inside_count : u32 = 0;
    for ri in 0..matrix.len() {
        let row = &matrix[ri];
        for ci in 0..row.len() {
            if is_inside((ri.try_into().unwrap(), ci.try_into().unwrap()), &max_maze, &matrix) {
                inside_count += 1;
            }
        }
    }

    println!("{}", inside_count);
}

fn step (loc: (i16, i16), dir: u8) -> (i16, i16) {
    let (r, c) = loc;
    match dir {
        0 => (r, c + 1),
        1 => (r - 1, c),
        2 => (r, c - 1),
        3 => (r + 1, c),
        _ => (r, c),
    }
}

fn is_inside((ri, ci): (i16, i16), maze: &HashSet<(i16, i16)>, matrix: &Vec::<Vec::<char>>) -> bool {
    let mut cross_count : u8 = 0;
    if !maze.contains(&(ri, ci)) {
        let row = &matrix[ri as usize];
        let mut recent_bend : char = '.';
        for ci_step in 0..ci {
            if maze.contains(&(ri, ci_step)) {
                let ch = row[ci_step as usize];
                match ch {
                    '|' => cross_count += 1,
                    'J' => {
                        cross_count += if recent_bend == 'F' { 1 } else { 0 };
                        recent_bend = '.';
                    },
                    '7' => {
                        cross_count += if recent_bend == 'L' { 1 } else { 0 };
                        recent_bend = '.';
                    },
                    'L' | 'F' => recent_bend = ch,
                    _ => {}
                };
            }
            else {
                recent_bend = '.';
            }
        }
    }

    let result = cross_count % 2 == 1;
    if result {
        println!("{}, {}", ri, ci);
    }

    cross_count % 2 == 1
}