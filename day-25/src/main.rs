use pom::parser::*;
use pom::Error;
use std::collections::HashSet;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Engine {
    points: Vec<Point>,
    constellations: Vec<HashSet<Point>>,
}

impl Engine {
    fn run(&mut self) -> u32 {
        while let Some(point) = self.points.pop() {
            let mut new_constellations = Vec::new();
            let mut new_constellation = HashSet::new();

            while let Some(constellation) = self.constellations.pop() {
                let mut point_is_in_constellation = false;
                for constellation_point in constellation.iter() {
                    if (point.w - constellation_point.w).abs()
                        + (point.x - constellation_point.x).abs()
                        + (point.y - constellation_point.y).abs()
                        + (point.z - constellation_point.z).abs()
                        <= 3
                    {
                        point_is_in_constellation = true;
                        break;
                    }
                }
                if point_is_in_constellation {
                    new_constellation = new_constellation.union(&constellation).cloned().collect();
                } else {
                    new_constellations.push(constellation);
                }
            }

            new_constellation.insert(point);
            new_constellations.push(new_constellation);

            self.constellations = new_constellations;
        }

        self.constellations.len() as u32
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

fn point<'a>() -> Parser<'a, u8, Point> {
    (number() + ((sym(b',') * number()) + ((sym(b',') * number()) + sym(b',') * number())))
        .map(|(w, (x, (y, z)))| Point { w, x, y, z })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (space() * point()).repeat(1..).map(|points| Engine {
        points,
        constellations: vec![],
    })
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;

    let constellation_count = engine.run();
    println!("Part 1: {} constellations are formed", constellation_count);

    Ok(())
}
