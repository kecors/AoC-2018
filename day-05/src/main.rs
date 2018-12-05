use std::io::{stdin, Read};

#[derive(Debug)]
struct Link {
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
struct Engine {
    units: Vec<char>,
    links: Vec<Link>,
}

impl Engine {
    fn new(polymer: &str) -> Engine {
        let units: Vec<char> = polymer.chars().collect();
        let mut links = Vec::with_capacity(units.len());

        links.push(Link { left: None, right: Some(1) });
        for j in 1..units.len()-1 {
            links.push(Link { left: Some(j-1), right: Some(j+1) });
        }
        links.push(Link { left: Some(units.len()-2), right: None });

        Engine { units, links }
    }

    fn solve(&mut self) -> u32 {
        let mut head = 0;
        let mut cursor = head;

        while let Some(next) = self.links[cursor].right {
            if do_react(self.units[cursor], self.units[next]) {
                match (self.links[cursor].left, self.links[next].right) {
                    (Some(left), Some(right)) => {
                        self.links[left].right = Some(right);
                        self.links[right].left = Some(left);

                        self.links[cursor].left = None;
                        self.links[cursor].right = None;
                        self.links[next].left = None;
                        self.links[next].right = None;

                        cursor = left;
                    },
                    (None, Some(right)) => {
                        self.links[right].left = None;

                        self.links[cursor].right = None;
                        self.links[next].left = None;
                        self.links[next].right = None;

                        head = right;
                        cursor = head;
                    },
                    (Some(left), None) => {
                        self.links[left].right = None;

                        self.links[cursor].left = None;
                        self.links[cursor].right = None;
                        self.links[next].left = None;

                        break;
                    },
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
        while let Some(next) = self.links[cursor].right {
            remaining_unit_count += 1;
            cursor = next;
        }

        remaining_unit_count
    }

    #[allow(dead_code)]
    fn display(&self) {
        let mut cursor = 0;

        while let None = self.links[cursor].right {
            cursor += 1;
        }

        println!("[{}] {:?} {}", cursor, self.links[cursor], self.units[cursor]);

        while let Some(next) = self.links[cursor].right {
            println!("[{}] {:?} {}", next, self.links[next], self.units[next]);
            cursor = next;
        }
    }
}

fn do_react(left: char, right: char) -> bool {
    if left == right {
        return false;
    }
    if left.to_uppercase().to_string() == right.to_uppercase().to_string() {
        return true;
    }

    false
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input.trim());

    let part1 = engine.solve();

    println!("Part 1: {} units remain after fully reacting the polymer", part1);
}
