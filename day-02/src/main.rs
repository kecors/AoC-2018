use std::collections::HashMap;
use std::io::{stdin, Read};

fn solve(ids: &[&str]) -> u32 {
    let mut two_count = 0;
    let mut three_count = 0;

    for id in ids.iter() {
        let mut hm = HashMap::new();

        for letter in id.chars() {
            *hm.entry(letter).or_insert(0) += 1;
        }

        let mut two_flag = false;
        let mut three_flag = false;
        for (_letter, &appearances) in hm.iter() {
            if appearances == 2 {
                two_flag = true;
            }
            if appearances == 3 {
                three_flag = true;
            }
        }

        if two_flag {
            two_count += 1;
        }
        if three_flag {
            three_count += 1;
        }
    }

    two_count * three_count
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let ids: Vec<&str> = input.lines().collect();

    let result = solve(&ids);
    println!("Part 1: the checksum is {}", result);
}
