use std::env;
use std::fs;
//use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let items = contents.lines();
    let mut acc : i32 = 0;
    let mut acc2 : i32 = 0;
    for item in items {
        let numbers = item.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        let mut extrapolated : Vec<_> = vec![numbers];
        let mut last_numbers = &extrapolated[extrapolated.len() - 1];
        while !is_null(last_numbers) {
            let mut next_numbers: Vec<i32> = vec![];
            for i in 1..last_numbers.len() {
                next_numbers.push(last_numbers[i] - last_numbers[i-1]);
            }

            extrapolated.push(next_numbers);
            last_numbers =  &extrapolated[extrapolated.len() - 1];
        }

        //dbg!(&extrapolated);

        let mut last_extra : i32 = *last_numbers.last().unwrap();
        let mut first_extra : i32 = last_numbers[0];
        let offset = extrapolated.len() - 1;
        for extra_index in 1..extrapolated.len() {
            println!("inter extra: {}", last_extra);
            let next_numbers = &extrapolated[offset - extra_index];
            //dbg!(next_numbers);
            let current_last : i32 = *next_numbers.last().unwrap();
            last_extra = current_last + last_extra;

            let current_first : i32 = anext_numbers[0];
            first_extra = current_first - first_extra;
        }
        println!("extrapolated {}, {}", last_extra, first_extra);
        acc += last_extra;
        acc2 += first_extra;
        //dbg!(&extrapolated);
    }

    println!("{}", acc);
    println!("{}", acc2);
}

fn is_null(numbers: &Vec<i32>) -> bool {
    //dbg!(numbers);
    numbers.iter().all(|n| *n == 0)
}