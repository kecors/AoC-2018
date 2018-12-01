use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result: i32 = input
        .lines()
        .fold(0, |acc, x| acc + x.trim().parse::<i32>().unwrap());
    println!("Part 1: the resulting frequency is {}", result);
}
