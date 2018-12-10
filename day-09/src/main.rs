use regex::Regex;
use std::collections::VecDeque;
use std::io::{stdin, Read};

fn solve(players: usize, last_marble: u32) -> u32 {
    let mut circle: VecDeque<u32> = VecDeque::new();
    circle.push_back(0);
    let mut marble_number: u32 = 1;
    let mut scores = vec![0; players];
    let mut player = 0;

    while marble_number <= last_marble {
        if marble_number % 23 == 0 {
            for _ in 0..6 {
                let marble = circle.pop_back().unwrap();
                circle.push_front(marble);
            }
            let current_marble = circle.pop_back().unwrap();
            let removed_marble = circle.pop_back().unwrap();
            scores[player] += marble_number + removed_marble;
            circle.push_back(current_marble);
        } else {
            let marble = circle.pop_front().unwrap();
            circle.push_back(marble);
            circle.push_back(marble_number);
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

    let part2 = solve(players, last_marble * 100);
    println!("Part 1: the winning elf's score is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(32, solve(9, 25));
    }

    #[test]
    fn test_10() {
        assert_eq!(8317, solve(10, 1618));
    }

    #[test]
    fn test_13() {
        assert_eq!(146373, solve(13, 7999));
    }

    #[test]
    fn test_17() {
        assert_eq!(2764, solve(17, 1104));
    }

    #[test]
    fn test_21() {
        assert_eq!(54718, solve(21, 6111));
    }

    #[test]
    fn test_30() {
        assert_eq!(37305, solve(30, 5807));
    }
}
