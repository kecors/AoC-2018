use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

#[derive(Debug)]
struct Requirement {
    before: char,
    after: char,
}

fn solve(requirements: &[Requirement]) -> String {
    let mut letter_hs = HashSet::new();
    for requirement in requirements.iter() {
        letter_hs.insert(requirement.before);
        letter_hs.insert(requirement.after);
    }
    let mut letters: Vec<char> = letter_hs.into_iter().collect();
    letters.sort();

    let mut obstructions: HashMap<char, HashSet<char>> = HashMap::new();
    for &letter in letters.iter() {
        obstructions.insert(letter, HashSet::new());
    }
    for requirement in requirements.iter() {
        if let Some(hs) = obstructions.get_mut(&requirement.after) {
            hs.insert(requirement.before);
        }
    }

    let mut result: Vec<char> = Vec::new();

    while result.len() < letters.len() {
        let mut letters_iter = letters.iter();
        let next_letter = loop {
            if let Some(letter) = letters_iter.next() {
                if result.contains(&letter) {
                    continue;
                }
                if obstructions[letter].is_empty() {
                    break *letter;
                }
            }
        };
        result.push(next_letter);
        for &letter in letters.iter() {
            if let Some(hs) = obstructions.get_mut(&letter) {
                hs.remove(&next_letter);
            }
        }
    }

    result.iter().collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut requirements = Vec::new();

    let rule = r"Step (\D) must be finished before step (\D) can begin.";
    let re = Regex::new(rule).unwrap();

    for capture in re.captures_iter(input.trim()) {
        let before = capture[1].chars().next().unwrap();
        let after = capture[2].chars().next().unwrap();
        requirements.push(Requirement { before, after });
    }

    let part1 = solve(&requirements);
    println!(
        "Part 1: the steps should be completed in this order: {}",
        part1
    );
}
