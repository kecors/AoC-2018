use pom::parser::*;
use pom::Error;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, Read};

#[derive(Debug)]
enum Token {
    North,
    East,
    South,
    West,
    Begin,
    Pipe,
    End,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Room {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Engine {
    tokens: Vec<Token>,
    sides: HashMap<Room, Vec<Room>>,
    distances: HashMap<Room, u32>,
}

impl Engine {
    fn determine_sides(&mut self) {
        use Token::*;

        let mut branches: Vec<Room> = Vec::new();
        let mut room = Room { x: 0, y: 0 };

        for token in self.tokens.iter() {
            match token {
                North => {
                    let sides = self.sides.entry(room).or_insert_with(Vec::new);
                    room.y += 1;
                    sides.push(room);
                }
                East => {
                    let sides = self.sides.entry(room).or_insert_with(Vec::new);
                    room.x += 1;
                    sides.push(room);
                }
                South => {
                    let sides = self.sides.entry(room).or_insert_with(Vec::new);
                    room.y -= 1;
                    sides.push(room);
                }
                West => {
                    let sides = self.sides.entry(room).or_insert_with(Vec::new);
                    room.x -= 1;
                    sides.push(room);
                }
                Begin => {
                    branches.push(room);
                }
                Pipe => {
                    room = branches.pop().expect("Branch must exist");
                    branches.push(room);
                }
                End => {
                    room = branches.pop().expect("Branch must exist");
                }
            }
        }
    }

    fn determine_distances(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back((Room { x: 0, y: 0 }, 0));

        while let Some((room, distance)) = queue.pop_front() {
            if let Entry::Vacant(v) = self.distances.entry(room) {
                v.insert(distance);

                if let Some(neighbors) = self.sides.get(&room) {
                    for neighbor in neighbors.iter() {
                        queue.push_back((*neighbor, distance + 1));
                    }
                }
            }
        }
    }
    fn max_distance(&self) -> u32 {
        *self.distances.values().max().unwrap()
    }
}

fn token<'a>() -> Parser<'a, u8, Token> {
    sym(b'N').map(|_| Token::North)
        | sym(b'E').map(|_| Token::East)
        | sym(b'S').map(|_| Token::South)
        | sym(b'W').map(|_| Token::West)
        | sym(b'|').map(|_| Token::Pipe)
        | sym(b'(').map(|_| Token::Begin)
        | sym(b')').map(|_| Token::End)
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (sym(b'^') * token().repeat(1..) - sym(b'$')).map(|tokens| Engine {
        tokens,
        sides: HashMap::new(),
        distances: HashMap::new(),
    })
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;
    engine.determine_sides();
    engine.determine_distances();

    println!(
        "Part 1: the largest number of required doors is {}",
        engine.max_distance()
    );

    Ok(())
}
