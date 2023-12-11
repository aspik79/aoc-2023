use std::cmp::Ordering;
use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let items = contents.lines();

    let mut tuples : Vec<(&str, u32)> = Vec::<_>::new();
    for item in items {
        let (card, score) = item.split_once(" ").expect("invalid line");
        let score = score.parse().expect("no number");
        tuples.push((card, score));
    }

    tuples.sort_by(|(x_card, _), (y_card, _)| my_cmp(x_card, y_card));

    let mut acc : u32 = 0;

    //let length:u32 = .try_into().unwrap();
    for i in 0..tuples.len() {
        let (_, v) = tuples[i];

        let rank: u32 = u32::try_from(i).unwrap() + 1;
        acc += rank * &v;
    }

    println!("{}", acc);

    tuples.sort_by(|(x_card, _), (y_card, _)| my_cmp_j(x_card, y_card));

    let mut acc : u32 = 0;

    //let length:u32 = .try_into().unwrap();
    for i in 0..tuples.len() {
        let (_, v) = tuples[i];

        let rank: u32 = u32::try_from(i).unwrap() + 1;
        acc += rank * &v;
    }

    // 249062580 too high
    println!("{}", acc);
}

fn my_cmp (x: &str, y: &str) -> Ordering {
    let arr: Vec::<&dyn Fn(&str) -> bool> =vec![&five_of_a_kind, &four_of_a_kind, &full_house, &three_of_a_kind, &two_pairs, &one_pair];
    
    for pred in arr {
        let xmatch = pred(x);
        let ymatch = pred(y);

        if xmatch || ymatch {
            if xmatch && ymatch {
                return compare_high(x, y);
            }

            if xmatch {
                return Ordering::Greater;
            }

            return Ordering::Less;
        }
    }

    compare_high(x, y)
}

fn my_cmp_j (x: &str, y: &str) -> Ordering {
    let arr: Vec::<&dyn Fn(&str) -> bool> =vec![&five_of_a_kind_j, &four_of_a_kind_j, &full_house_j, &three_of_a_kind_j, &two_pairs_j, &one_pair_j];
    
    for pred in arr {
        let xmatch = pred(x);
        let ymatch = pred(y);

        if xmatch || ymatch {
            if xmatch && ymatch {
                return compare_high_j(x, y);
            }

            if xmatch {
                return Ordering::Greater;
            }

            return Ordering::Less;
        }
    }

    compare_high_j(x, y)
}

fn find_counts(s: &str) -> Vec<u32> {
    let map1 = s.chars()
    .fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    map1.values().cloned().collect()
}

fn five_of_a_kind(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().any(|count| *count == 5);

    if r {
        //println!("5 of a kind: {}", s);
    }
    r
}

fn four_of_a_kind(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().any(|count| *count == 4);

    if r {
        //println!("4 of a kind: {}", s);
    }
    r
}

fn full_house(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().any(|count| *count == 3) && c.iter().any(|count| *count == 2);

    if r {
        //println!("full house: {}", s);
    }
    r
}

fn three_of_a_kind(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().any(|count| *count == 3);

    if r {
        //println!("3 of a kind: {}", s);
    }
    r
}

fn two_pairs(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().filter(|count| (**count) == 2).count() == 2;

    if r {
        //println!("two pairs: {}", s);
    }
    r
}

fn one_pair(s: &str) -> bool {
    let c = find_counts(s);
    let r = c.iter().any(|count| *count == 2);

    if r {
        //println!("2 of a kind: {}", s);
    }
    r
}

fn compare_high(x: &str, y: &str) -> Ordering {
    let s = "AKQJT98765432";

    let mut xci: std::str::Chars<'_> = x.chars();
    let mut yci: std::str::Chars<'_> = y.chars();

    for _ in 0..x.len() {
        let xc = xci.next().unwrap();
        let yc: char = yci.next().unwrap();

        if xc != yc {
            let xi = s.find(xc).unwrap();
            let yi = s.find(yc).unwrap();
            return if xi < yi { Ordering::Greater } else { Ordering::Less };
        }
    }
    Ordering::Equal
}

fn five_of_a_kind_j(s: &str) -> bool {
    let s_without_j: String = s.chars().filter(|c| *c != 'J').collect();

    let c = find_counts(&s_without_j);
    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();
    let r = j_count == 5 || c.iter().any(|count| *count >= 5 - j_count);

    if r {
        //println!("5 of a kind: {}", s);
    }
    r
}

fn four_of_a_kind_j(s: &str) -> bool {
    let s_without_j: String = s.chars().filter(|c| *c != 'J').collect();

    let c = find_counts(&s_without_j);
    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();
    let r = c.iter().any(|count| *count >= 4 - j_count);

    if r {
        //println!("4 of a kind: {}", s);
    }
    r
}

fn full_house_j(s: &str) -> bool {
    let s_without_j: String = s.chars().filter(|c| *c != 'J').collect();

    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();

    let r: bool = j_count >= 2 && one_pair(&s_without_j)
        || j_count == 1 && (three_of_a_kind(&s_without_j) || two_pairs(&s_without_j))
        || full_house(s);

    if r {
        //println!("full house: {}", s);
    }
    r
}

fn three_of_a_kind_j(s: &str) -> bool {
    let s_without_j: String = s.chars().filter(|c| *c != 'J').collect();

    let c = find_counts(&s_without_j);
    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();
    let r = c.iter().any(|count| *count >= 3 - j_count);

    if r {
        //println!("3 of a kind: {}", s);
    }
    r
}

fn two_pairs_j(s: &str) -> bool {
    let s_without_j: String = s.chars().filter(|c| *c != 'J').collect();

    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();

    let r = j_count >= 2
        || j_count == 1 && one_pair(&s_without_j)
        || two_pairs(s);

    if r {
        //println!("two pairs: {}", s);
    }
    r
}

fn one_pair_j(s: &str) -> bool {
    let j_count = u32::try_from(s.chars().filter(|c| *c == 'J').count()).unwrap();
    
    let r = j_count > 0 || one_pair(s);

    if r {
        //println!("one pair: {}", s);
    }
    r
}

fn compare_high_j(x: &str, y: &str) -> Ordering {
    let s = "AKQT98765432J";

    let mut xci: std::str::Chars<'_> = x.chars();
    let mut yci: std::str::Chars<'_> = y.chars();

    for _ in 0..x.len() {
        let xc = xci.next().unwrap();
        let yc: char = yci.next().unwrap();

        if xc != yc {
            let xi = s.find(xc).unwrap();
            let yi = s.find(yc).unwrap();
            return if xi < yi { Ordering::Greater } else { Ordering::Less };
        }
    }
    Ordering::Equal
}
