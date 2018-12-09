use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

#[derive(Debug)]
struct Delay {
    seconds: u32,
}

impl Into<Delay> for char {
    fn into(self) -> Delay {
        let seconds = match self {
            'A' => 61,
            'B' => 62,
            'C' => 63,
            'D' => 64,
            'E' => 65,
            'F' => 66,
            'G' => 67,
            'H' => 68,
            'I' => 69,
            'J' => 70,
            'K' => 71,
            'L' => 72,
            'M' => 73,
            'N' => 74,
            'O' => 75,
            'P' => 76,
            'Q' => 77,
            'R' => 78,
            'S' => 79,
            'T' => 80,
            'U' => 81,
            'V' => 82,
            'W' => 83,
            'X' => 84,
            'Y' => 85,
            'Z' => 86,
            _ => 0,
        };
        Delay { seconds }
    }
}

#[derive(Debug)]
struct Requirement {
    before: char,
    after: char,
}

fn solve(workers: usize, requirements: &[Requirement]) -> (String, u32) {
    let mut letter_hs = HashSet::new();
    for requirement in requirements.iter() {
        letter_hs.insert(requirement.before);
        letter_hs.insert(requirement.after);
    }
    let mut letters: Vec<char> = letter_hs.into_iter().collect();
    letters.sort();

    let mut obstructions: HashMap<char, Option<HashSet<char>>> = HashMap::new();
    for &letter in letters.iter() {
        obstructions.insert(letter, Some(HashSet::new()));
    }
    for requirement in requirements.iter() {
        if let Some(hs_opt) = obstructions.get_mut(&requirement.after) {
            if let Some(hs) = hs_opt {
                hs.insert(requirement.before);
            }
        }
    }

    let mut seconds = 0;
    let mut delays: Vec<Option<(u32, char)>> = vec![None; 5];
    let mut order: Vec<char> = Vec::new();

    loop {
        for worker in 0..workers {
            if None != delays[worker] {
                continue;
            }
            let mut next_letter_opt = None;
            for letter in letters.iter() {
                if order.contains(&letter) {
                    continue;
                }
                if let Some(obstruction_opt) = obstructions.get_mut(&letter) {
                    if let Some(obstruction) = obstruction_opt {
                        if obstruction.is_empty() {
                            next_letter_opt = Some(*letter);
                            *obstruction_opt = None;
                            break;
                        }
                    }
                }
            }
            if let Some(next_letter) = next_letter_opt {
                let delay: Delay = next_letter.into();
                delays[worker] = Some((delay.seconds, next_letter));
            }
        }

        if order.len() == letters.len() {
            break;
        }

        seconds += 1;

        for worker in 0..workers {
            if let Some((mut delay, next_letter)) = delays[worker] {
                delay -= 1;

                if delay == 0 {
                    order.push(next_letter);

                    for &letter in letters.iter() {
                        if let Some(hs_opt) = obstructions.get_mut(&letter) {
                            if let Some(hs) = hs_opt {
                                hs.remove(&next_letter);
                            }
                        }
                    }

                    delays[worker] = None;
                } else {
                    delays[worker] = Some((delay, next_letter));
                }
            }
        }
    }

    (order.iter().collect(), seconds)
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

    let (order, _seconds) = solve(1, &requirements);
    println!(
        "Part 1: the steps should be completed in this order: {}",
        order
    );

    let (_order, seconds) = solve(5, &requirements);
    println!("Part 2: It will take {} seconds to complete", seconds);
}
