use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let deltas: Vec<i32> = input
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let result: i32 = deltas.iter().sum();
    println!("Part 1: the resulting frequency is {}", result);

    let mut frequency = 0;
    let mut hs = HashSet::new();

    for delta in deltas.iter().cycle() {
        frequency += delta;
        if !hs.insert(frequency) {
            break;
        }
    }

    println!(
        "Part 2: the first frequency the device reaches twice is {}",
        frequency
    );
}
