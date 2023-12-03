use std::env;
use std::fs;
//use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data = Vec::<Vec<char>>::new();
    let iter = contents.lines();

    for item in iter {
        data.push(item.chars().collect());
    }

    let mut acc = 0;

    for ri in 0..data.len() {
        let row = &data[ri];

        let mut value = 0;
        let mut is_part = false;

        for ci in 0..row.len() {
            let c = row[ci];

            if c.is_numeric() {
                let d = c.to_digit(10).unwrap();
                value = value * 10 + d;

                if !is_part {
                    if ri > 0 {
                        let up_row = &data[ri - 1];
                        if ci > 0 {
                            let c = up_row[ci - 1];
                            is_part = is_part || c != '.';
                        }

                        let c = up_row[ci];
                        is_part = is_part || c != '.';

                        if ci < up_row.len() - 1 {
                            let c = up_row[ci + 1];
                            is_part = is_part || c != '.';
                        }
                    }

                    if ci > 0 {
                        let c = row[ci - 1];
                        is_part = is_part || c != '.' && !c.is_numeric();
                    }

                    if ci < row.len() - 1 {
                        let c = row[ci + 1];
                        is_part = is_part || c != '.' && !c.is_numeric();
                    }

                    if ri < data.len() - 1 {
                        let down_row = &data[ri + 1];
                        if ci > 0 {
                            let c = down_row[ci - 1];
                            is_part = is_part || c != '.';
                        }

                        let c = down_row[ci];
                        is_part = is_part || c != '.';

                        if ci < down_row.len() - 1 {
                            let c = down_row[ci + 1];
                            is_part = is_part || c != '.';
                        }
                    }

                }
            }
            else {
                if is_part {
                    acc += value;
                }

                is_part = false;
                value = 0;
            }
        }

        if is_part {
            acc += value;
        }
    }

    println!("{}", acc);

    // part 2
    acc = 0;
    for ri in 0..data.len() {
        let row = &data[ri];

        for ci in 0..row.len() {
            let c = row[ci];
            if c == '*' {
                let mut neighbours : Vec<u32> = Vec::new();

                if ri > 0 {
                    // parse number above
                    let up_row = &data[ri - 1];

                    if up_row[ci].is_numeric() {
                        let value = parse_around(up_row, ci);
                        neighbours.push(value);
                    }
                    else {
                        // parse potential numbers starting up right
                        let value = parse_right(up_row, ci);
                        neighbours.push(value);

                        // parse potential numbers ending up left
                        let value = parse_left(up_row, ci);
                        neighbours.push(value);
                    }
                }

                if ri < data.len() - 1 {
                    // parse number below
                    let down_row = &data[ri + 1];

                    if down_row[ci].is_numeric() {
                        let value = parse_around(down_row, ci);
                        neighbours.push(value);
                    }
                    else {
                        // parse potential numbers starting down right
                        let value = parse_right(down_row, ci);
                        neighbours.push(value);

                        // parse potential numbers ending down left
                        let value = parse_left(down_row, ci);
                        neighbours.push(value);
                    }
                }

                // parse potential numbers starting right
                let value = parse_right(row, ci);
                neighbours.push(value);

                // parse potential numbers ending left
                let value = parse_left(row, ci);
                neighbours.push(value);
                
                let neighbours_filtered : Vec<_> = neighbours.iter().filter(|&n| n > &0).collect();
                if neighbours_filtered.len() == 2 {
                    acc += neighbours_filtered[0] * neighbours_filtered[1];
                }
            }
        }
    }

    println!("{}", acc);
}

fn parse_around(row: &Vec<char>, i: usize) -> u32 {
    let mut low = i;
    while low > 0 && row[low - 1].is_numeric() {
        low -= 1;
    }

    let mut high = i;
    while high < row.len() && row[high].is_numeric() {
        high += 1;
    }

    let mut value = 0;
    for i in low..high {
        value = 10 * value + row[i].to_digit(10).unwrap();
    }

    value
}

fn parse_left(row: &Vec<char>, i: usize) -> u32 {
    let mut value = 0;

    if i > 0 && row[i - 1].is_numeric() {
        let high = i;
        let mut low = i;
        while low > 0 && row[low - 1].is_numeric() {
            low -= 1;
        }

        for i in low..high {
            value = 10 * value + row[i].to_digit(10).unwrap();
        }
    }

    value
}


fn parse_right(row: &Vec<char>, i: usize) -> u32 {
    let mut value = 0;

    let low = i + 1;
    let mut high = i + 1;
    while high < row.len() && row[high].is_numeric() {
        high += 1;
    }

    for i in low..high {
        value = 10 * value + row[i].to_digit(10).unwrap();
    }

    value
}