use std::collections::HashMap;
use std::io::{stdin, Read};

fn solve_part1(ids: &[&str]) -> u32 {
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

fn solve_part2(ids: &[&str]) -> String {
    let ids_chars: Vec<Vec<char>> = ids.iter().map(|x| x.chars().collect()).collect();

    for j in 0..ids_chars.len() {
        for k in j + 1..ids_chars.len() {
            let mut common_chars = Vec::new();
            for m in 0..ids_chars[j].len() {
                if ids_chars[j][m] == ids_chars[k][m] {
                    common_chars.push(ids_chars[j][m]);
                }
            }
            if common_chars.len() == ids_chars[j].len() - 1 {
                return common_chars.iter().collect();
            }
        }
    }

    panic!("No solution available");
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let ids: Vec<&str> = input.lines().collect();

    println!("Part 1: the checksum is {}", solve_part1(&ids));
    println!("Part 2: the common letters are {}", solve_part2(&ids));
}
