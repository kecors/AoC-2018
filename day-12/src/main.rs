use pom::parser::*;
use pom::Error;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Pot {
    number: i32,
    plant: bool,
}

#[derive(Debug)]
struct Note {
    neighbors: Vec<bool>,
    next_generation: bool,
}

#[derive(Debug)]
struct Engine {
    pots: VecDeque<Pot>,
    note_hm: HashMap<Vec<bool>, bool>,
}

impl Engine {
    fn new(initial_state: Vec<bool>, notes: Vec<Note>) -> Engine {
        let mut pots = VecDeque::new();
        for (n, plant) in initial_state.into_iter().enumerate() {
            pots.push_back(Pot {
                number: n as i32,
                plant,
            });
        }
        let mut note_hm = HashMap::new();
        for note in notes {
            note_hm.insert(note.neighbors, note.next_generation);
        }

        Engine { pots, note_hm }
    }

    fn next_generation(&mut self) {
        // Pad the queue with plantless pots to enable processing the end pots
        let front_number = if let Some(pot) = self.pots.front() {
            pot.number
        } else {
            unreachable!("Impossible if any plants remain");
        };
        let back_number = if let Some(pot) = self.pots.back() {
            pot.number
        } else {
            unreachable!("Impossible if any plants remain");
        };
        for x in 1..=4 {
            self.pots.push_front(Pot {
                number: front_number - x,
                plant: false,
            });
            self.pots.push_back(Pot {
                number: back_number + x,
                plant: false,
            });
        }

        // Calculate next generation plant status for each non-edge pot.
        // Note that j + 2 is the pot under consideration.
        let mut new_pots = VecDeque::new();
        for j in 0..self.pots.len() - 4 {
            let mut neighbor_key = Vec::new();
            for k in 0..5 {
                neighbor_key.push(self.pots[j + k].plant);
            }
            let plant = if let Some(plant) = self.note_hm.get(&neighbor_key) {
                *plant
            } else {
                false
            };
            new_pots.push_back(Pot {
                number: self.pots[j + 2].number,
                plant,
            });
        }

        // Remove plantless pots from the front of the queue
        let pot_to_restore = loop {
            if let Some(pot) = new_pots.pop_front() {
                if pot.plant {
                    break pot;
                }
            }
        };
        new_pots.push_front(pot_to_restore);

        // Remove plantless pots from the back of the queue
        let pot_to_restore = loop {
            if let Some(pot) = new_pots.pop_back() {
                if pot.plant {
                    break pot;
                }
            }
        };
        new_pots.push_back(pot_to_restore);

        self.pots = new_pots;
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;

        for pot in self.pots.iter() {
            if pot.plant {
                sum += pot.number;
            }
        }

        sum
    }

    fn range(&self) -> (i32, i32) {
        let front_number = if let Some(pot) = self.pots.front() {
            pot.number
        } else {
            unreachable!("Impossible if any plants remain");
        };
        let back_number = if let Some(pot) = self.pots.back() {
            pot.number
        } else {
            unreachable!("Impossible if any plants remain");
        };

        (front_number, back_number)
    }

    fn pattern(&self) -> String {
        let mut pattern = String::new();

        for pot in self.pots.iter() {
            pattern.push(if pot.plant { '#' } else { '.' });
        }

        pattern
    }

    #[allow(dead_code)]
    fn display(&self) {
        for pot in self.pots.iter() {
            print!("{} ", pot.number);
        }
        println!();
        for pot in self.pots.iter() {
            print!("{}", if pot.plant { "#" } else { "." });
        }
        println!();
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn plant<'a>() -> Parser<'a, u8, bool> {
    sym(b'#').map(|_| true) | sym(b'.').map(|_| false)
}

fn initial_state<'a>() -> Parser<'a, u8, Vec<bool>> {
    let prefix = seq(b"initial state: ").discard();
    let plants = plant().repeat(1..);
    prefix * plants
}

fn note<'a>() -> Parser<'a, u8, Note> {
    (plant().repeat(5) + skip(4) * plant()).map(|(neighbors, next_generation)| Note {
        neighbors,
        next_generation,
    })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    let notes = (space() * note()).repeat(1..);
    (initial_state() + notes).map(|(initial_state, notes)| Engine::new(initial_state, notes))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine_p1 = engine().parse(input.as_bytes())?;
    for _ in 0..20 {
        engine_p1.next_generation();
    }
    let part1 = engine_p1.sum();
    println!(
        "Part 1: After 20 generations, the sum of the numbers of all pots which contain a plant is {}",
        part1
    );

    let mut engine_p2 = engine().parse(input.as_bytes())?;
    let mut patterns = HashMap::new();

    // The following block of code allows the observation that the pattern
    // of plants remains stable after the first 158 generations. For each
    // subsequent generation, the pattern shifts to the right by one pot and the
    // sum increases by 86.
    for x in 0..200 {
        if x > 157 {
            let p = patterns.entry(engine_p2.pattern()).or_insert_with(Vec::new);
            p.push((x, engine_p2.range(), engine_p2.sum()));
        }

        engine_p2.next_generation();
        println!("[{}] sum = {}", x, engine_p2.sum());
        engine_p2.display();
        println!();
    }
    println!("patterns = {:#?}", patterns);

    let part2: u64 = (50_000_000_000 - 158) * 86 + 16002;
    println!(
        "Part 2: After fifty billion generations, the sum of the numbers of all pots which contain a plant is {}",
        part2
    );

    Ok(())
}
