use std::io::{stdin, Read};

#[derive(Debug)]
struct Link {
    left: Option<usize>,
    right: Option<usize>,
}

fn do_react(left: char, right: char) -> bool {
    if left == right {
        return false;
    }
    if left.to_ascii_uppercase() == right.to_ascii_uppercase() {
        return true;
    }

    false
}

fn react_polymer(units: &[char]) -> u32 {
    let mut links = Vec::with_capacity(units.len());

    links.push(Link {
        left: None,
        right: Some(1),
    });
    for j in 1..units.len() - 1 {
        links.push(Link {
            left: Some(j - 1),
            right: Some(j + 1),
        });
    }
    links.push(Link {
        left: Some(units.len() - 2),
        right: None,
    });

    let mut head = 0;
    let mut cursor = head;

    while let Some(next) = links[cursor].right {
        if do_react(units[cursor], units[next]) {
            match (links[cursor].left, links[next].right) {
                (Some(left), Some(right)) => {
                    links[left].right = Some(right);
                    links[right].left = Some(left);

                    links[cursor].left = None;
                    links[cursor].right = None;
                    links[next].left = None;
                    links[next].right = None;

                    cursor = left;
                }
                (None, Some(right)) => {
                    links[right].left = None;

                    links[cursor].right = None;
                    links[next].left = None;
                    links[next].right = None;

                    head = right;
                    cursor = head;
                }
                (Some(left), None) => {
                    links[left].right = None;

                    links[cursor].left = None;
                    links[cursor].right = None;
                    links[next].left = None;

                    break;
                }
                (None, None) => {
                    return 0;
                }
            }
        } else {
            cursor = next;
        }
    }

    let mut remaining_unit_count = 1;

    cursor = head;
    while let Some(next) = links[cursor].right {
        remaining_unit_count += 1;
        cursor = next;
    }

    remaining_unit_count
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let units: Vec<char> = input.trim().chars().collect();

    let remaining_unit_count = react_polymer(&units);
    println!(
        "Part 1: {} units remain after fully reacting the polymer",
        remaining_unit_count
    );

    let mut unit_types: Vec<char> = units.iter().map(|x| x.to_ascii_lowercase()).collect();
    unit_types.sort();
    unit_types.dedup();

    let mut min_remaining_unit_count = u32::max_value();
    for unit_type in unit_types {
        let filtered_units: Vec<char> = units
            .clone()
            .into_iter()
            .filter(|x| x.to_ascii_lowercase() != unit_type)
            .collect();
        let remaining_unit_count = react_polymer(&filtered_units);
        if remaining_unit_count < min_remaining_unit_count {
            min_remaining_unit_count = remaining_unit_count;
        }
    }
    println!(
        "Part 2: the length of the shortest polymer is {}",
        min_remaining_unit_count
    );
}
