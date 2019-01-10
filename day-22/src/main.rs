use pom::parser::*;
use pom::Error;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Engine {
    depth: u32,
    target: Coordinate,
    erosion_levels: Vec<Vec<u32>>,
}

impl Engine {
    fn new(depth: u32, target: Coordinate) -> Engine {
        let mut erosion_levels: Vec<Vec<u32>> = Vec::new();

        for y in 0..=target.y {
            let mut row = Vec::new();
            for x in 0..=target.x {
                let geologic_index = if x == 0 && y == 0 {
                    0
                } else if x == target.x && y == target.y {
                    0
                } else if y == 0 {
                    x as u32 * 16807
                } else if x == 0 {
                    y as u32 * 48271
                } else {
                    row[x - 1] * erosion_levels[y - 1][x]
                };

                let erosion_level = (geologic_index + depth) % 20183;

                row.push(erosion_level);
            }
            erosion_levels.push(row);
        }

        Engine {
            depth,
            target,
            erosion_levels,
        }
    }

    fn display(&self) {
        for y in 0..self.erosion_levels.len() {
            for x in 0..self.erosion_levels[0].len() {
                if x == 0 && y == 0 {
                    print!("M");
                    continue;
                }
                if x == self.target.x && y == self.target.y {
                    print!("T");
                    continue;
                }
                print!(
                    "{}",
                    match self.erosion_levels[y][x] % 3 {
                        0 => '.',
                        1 => '=',
                        2 => '|',
                        _ => panic!(),
                    }
                );
            }
            println!();
        }
    }

    fn risk_level(&self) -> u32 {
        let mut risk_level = 0;

        for y in 0..self.erosion_levels.len() {
            for x in 0..self.erosion_levels[0].len() {
                risk_level += self.erosion_levels[y][x] % 3;
            }
        }

        risk_level
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, usize> {
    let number = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| usize::from_str_radix(s, 10))
}

fn depth<'a>() -> Parser<'a, u8, u32> {
    space() * seq(b"depth:") * space() * number().map(|x| x as u32)
}

fn target<'a>() -> Parser<'a, u8, Coordinate> {
    space()
        * seq(b"target:")
        * space()
        * (number() + (sym(b',') * number())).map(|(x, y)| Coordinate { x, y })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (depth() + target()).map(|(depth, target)| Engine::new(depth, target))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = engine().parse(input.as_bytes())?;
    engine.display();

    println!("Part 1: the total risk level is {}", engine.risk_level());

    Ok(())
}
