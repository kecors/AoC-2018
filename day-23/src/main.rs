// For this problem I implemented essentially the algorithm described here:
//
// https://elixirforum.com/t/advent-of-code-day-23/18938/5
//
// sasajuric said:
//
// So let’s say that we split the entire area into four large cubes, and 
// then compute the amount of nanobots each cube intersects with. We’ll 
// store all the cubes into a priority queue. Then, from the queue we pull 
// the cube which intersects with the most nanobots. If there are multiple 
// cubes with the same best score, we pick the one closest to the origin 
// (0, 0, 0). We then divide that cube, compute the intersection scores 
// for each subcube, and put the subcubes into the queue. Then rinse and 
// repeat. The first cube of size 1 that we pull is the solution.
//

use pom::parser::*;
use pom::Error;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Cube {
    extent: i32,
    base_x: i32,
    base_y: i32,
    base_z: i32,
}

impl Cube {
    fn new(extent: i32) -> Cube {
        Cube {
            extent,
            base_x: -(extent / 2),
            base_y: -(extent / 2),
            base_z: -(extent / 2),
        }
    }

    fn subdivide(&self) -> Vec<Cube> {
        let deltas = [
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1),
        ];
        let extent = self.extent / 2;
        let mut cubes = Vec::new();

        for (dx, dy, dz) in deltas.iter() {
            cubes.push(Cube {
                extent,
                base_x: self.base_x + (dx * extent),
                base_y: self.base_y + (dy * extent),
                base_z: self.base_z + (dz * extent),
            });
        }

        cubes
    }
}

#[derive(Debug)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Nanobot {
    fn in_range_of_nanobot(&self, nanobot: &Nanobot) -> bool {
        let distance =
            (nanobot.x - self.x).abs() + (nanobot.y - self.y).abs() + (nanobot.z - self.z).abs();

        distance <= nanobot.r
    }

    fn in_range_of_cube(&self, cube: &Cube) -> bool {
        let distance_x = if self.x < cube.base_x {
            (cube.base_x - self.x).abs()
        } else if self.x > (cube.base_x + cube.extent - 1) {
            (self.x - (cube.base_x + cube.extent - 1)).abs()
        } else {
            0
        };

        let distance_y = if self.y < cube.base_y {
            (cube.base_y - self.y).abs()
        } else if self.y > (cube.base_y + cube.extent - 1) {
            (self.y - (cube.base_y + cube.extent - 1)).abs()
        } else {
            0
        };

        let distance_z = if self.z < cube.base_z {
            (cube.base_z - self.z).abs()
        } else if self.z > (cube.base_z + cube.extent - 1) {
            (self.z - (cube.base_z + cube.extent - 1)).abs()
        } else {
            0
        };

        self.r >= distance_x + distance_y + distance_z
    }
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
            if nanobot.in_range_of_nanobot(&strongest) {
                in_range_count += 1;
            }
        }

        in_range_count
    }

    fn run(&self) -> Cube {
        let mut cubes = BinaryHeap::new();

        cubes.push((self.nanobots.len(), Reverse(self.generate_starting_cube())));

        while let Some((_nanobot_count, Reverse(cube))) = cubes.pop() {
            if cube.extent == 1 {
                return cube;
            }
            for subcube in cube.subdivide() {
                cubes.push((self.nanobot_count(&subcube), Reverse(subcube)));
            }
        }
        panic!("A cube with extent 1 should have been found");
    }

    fn nanobot_count(&self, cube: &Cube) -> usize {
        let mut nanobot_count = 0;

        for nanobot in self.nanobots.iter() {
            if nanobot.in_range_of_cube(&cube) {
                nanobot_count += 1;
            }
        }

        nanobot_count
    }

    fn generate_starting_cube(&self) -> Cube {
        let (mut min_x, mut max_x) = (i32::max_value(), i32::min_value());
        let (mut min_y, mut max_y) = (i32::max_value(), i32::min_value());
        let (mut min_z, mut max_z) = (i32::max_value(), i32::min_value());

        for nanobot in self.nanobots.iter() {
            if nanobot.x < min_x {
                min_x = nanobot.x;
            }
            if nanobot.x > max_x {
                max_x = nanobot.x;
            }
            if nanobot.y < min_y {
                min_y = nanobot.y;
            }
            if nanobot.y > max_y {
                max_y = nanobot.y;
            }
            if nanobot.z < min_z {
                min_z = nanobot.z;
            }
            if nanobot.z > max_z {
                max_z = nanobot.z;
            }
        }

        let maximum_boundary = *[min_x, max_x, min_y, max_y, min_z, max_z]
            .iter()
            .max()
            .unwrap();

        let mut extent = 1;
        while extent < maximum_boundary {
            extent *= 2;
        }

        Cube::new(extent)
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

    let engine = engine().parse(input.as_bytes())?;
    println!(
        "Part 1: {} nanobots are in range of the strongest nanobot",
        engine.in_range_of_strongest()
    );

    let cube = engine.run();
    let shortest_distance = cube.base_x.abs() + cube.base_y.abs() + cube.base_z.abs();
    println!(
        "Part 2: the shortest manhattan distance is {}",
        shortest_distance
    );

    Ok(())
}
