use regex::Regex;
use std::io::{stdin, Read};

fn solve(players: usize, last_marble: u32) -> u32 {
    let mut circle: Vec<u32> = vec![0];
    let mut current_position: usize = 0;
    let mut marble_number: u32 = 1;
    let mut scores = vec![0; players];
    let mut player = 0;

    while marble_number <= last_marble {
        if marble_number % 23 == 0 {
            let removal_position = (current_position + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(removal_position);
            scores[player] += marble_number + removed_marble;
            current_position = (removal_position + circle.len()) % circle.len();
        } else {
            current_position = (current_position + 2) % circle.len();
            circle.insert(current_position, marble_number);
        }
        marble_number += 1;
        player = (player + 1) % players;
    }

    scores.into_iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let rule = r"^(\d+) players; last marble is worth (\d+) points$";
    let re = Regex::new(rule).unwrap();

    let captures = re.captures(&input.trim()).unwrap();
    let players: usize = captures[1].parse().unwrap();
    let last_marble: u32 = captures[2].parse().unwrap();

    let part1 = solve(players, last_marble);
    println!("Part 1: the winning elf's score is {}", part1);
}
