use pom::parser::*;
use pom::Error;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

#[derive(Debug)]
struct Engine {
    nanobots: Vec<Nanobot>,
}

impl Engine {
    fn in_range_of_strongest(&self) -> usize {
        let strongest = self
            .nanobots
            .iter()
            .max_by(|a, b| a.r.cmp(&b.r))
            .expect("One or more nanobots required");

        let mut in_range_count = 0;

        for nanobot in self.nanobots.iter() {
            let distance = (strongest.x - nanobot.x).abs()
                + (strongest.y - nanobot.y).abs()
                + (strongest.z - nanobot.z).abs();
            if distance <= strongest.r {
                in_range_count += 1;
            }
        }

        in_range_count
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, i32> {
    let integer = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    let number = sym(b'-').opt() + integer;
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| i32::from_str_radix(s, 10))
}

fn nanobot<'a>() -> Parser<'a, u8, Nanobot> {
    (seq(b"pos=<") * number()
        + ((sym(b',') * number()) + ((sym(b',') * number()) + seq(b">, r=") * number())))
    .map(|(x, (y, (z, r)))| Nanobot { x, y, z, r })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (space() * nanobot())
        .repeat(1..)
        .map(|nanobots| Engine { nanobots })
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine_p1 = engine().parse(input.as_bytes())?;
    println!(
        "Part 1: {} nanobots are in range of the strongest nanobot",
        engine_p1.in_range_of_strongest()
    );

    Ok(())
}
