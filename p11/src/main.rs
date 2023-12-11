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

    let items : Vec<_> = contents.lines().collect();

    let mut padded_contents: Vec<_> = Vec::<Vec::<char>>::new();
    let mut contents: Vec<_> = Vec::<Vec::<char>>::new();
    let mut empty_row_indexes: Vec<_> = Vec::<usize>::new();
   
    for ri in 0..items.len() {
        let item = items[ri];
        let characters : Vec<_> = item.chars().collect();
        contents.push(characters.clone());
        padded_contents.push(characters.clone());
        if !characters.iter().any(|ch| *ch == '#') {
            empty_row_indexes.push(ri);
            padded_contents.push(characters.clone());
        }
    }

    dbg!(&empty_row_indexes);
    let mut empty_col_indexes: Vec<_> = Vec::<usize>::new();
    for ci in 0.. padded_contents[0].len() {
        if !padded_contents.iter().any(|row| row[ci] == '#') {
            empty_col_indexes.push(ci);
        }
    }

    dbg!(&empty_col_indexes);
    let mut ci = padded_contents[0].len();
    while ci > 0 {
        if !padded_contents.iter_mut().any(|row| row[ci - 1] == '#') {
            for r in padded_contents.iter_mut() {
                r.insert(ci - 1, '.');
            }
        }

        ci -= 1;
    }

    let mut galaxy_locs : Vec<_> = Vec::<(usize, usize)>::new();
//    for ri in 0..padded_contents.len() {
      for ri in 0..contents.len() {
        //let row = &padded_contents[ri];
        let row = &contents[ri];
        for ci in 0..row.len() {
            if row[ci] == '#' {
                galaxy_locs.push((ri, ci));
            }
        }
    }

    //
    //dbg!(&galaxy_locs);

    /*
    let mut distance_acc: i32 = 0;
    for i in 0..galaxy_locs.len() {
        let (r1, c1) = galaxy_locs[i];
        for j in i+1..galaxy_locs.len() {
            let (r2, c2) = galaxy_locs[j];
            let dr = (r1 as i32 - r2 as i32).abs();
            let dc = (c1 as i32 - c2 as i32).abs();
            distance_acc += dr + dc;
        }
    }
    */

    let distance_acc = find_distances(&galaxy_locs, &empty_row_indexes, &empty_col_indexes, 1);
    println!("{}", distance_acc);

    let distance_acc = find_distances(&galaxy_locs, &empty_row_indexes, &empty_col_indexes, 9);
    println!("{}", distance_acc);

    let distance_acc = find_distances(&galaxy_locs, &empty_row_indexes, &empty_col_indexes, 99);
    println!("{}", distance_acc);

    let distance_acc = find_distances(&galaxy_locs, &empty_row_indexes, &empty_col_indexes, 999999);
    println!("{}", distance_acc);
}

fn find_distances(galaxy_locs: &Vec<(usize, usize)>, empty_ri: &Vec<usize>, empty_ci: &Vec<usize>, skip: i64) -> i64 {
    let mut distance_acc: i64 = 0;
    for i in 0..galaxy_locs.len() {
        let (r1, c1) = galaxy_locs[i];
        for j in i..galaxy_locs.len() {
            let (r2, c2) = galaxy_locs[j];

            let empty_rows_between = empty_ri.iter().filter(|&ri| r1 < *ri && *ri < r2 || r1 > *ri && *ri > r2).collect::<Vec<_>>().len() as i64;
            let dr = (r1 as i64 - r2 as i64).abs() + empty_rows_between * skip;

            let empty_cols_between = empty_ci.iter().filter(|&ci| c1 < *ci && *ci < c2 || c1 > *ci && *ci > c2).collect::<Vec<_>>().len() as i64;
            let dc = (c1 as i64 - c2 as i64).abs() + empty_cols_between * skip;

            if i == j {
                //dbg!(&(dr, dc));
            }

            distance_acc += dr + dc;
        }
    }

    distance_acc
}