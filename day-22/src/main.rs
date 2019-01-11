use pom::parser::*;
use pom::Error;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn adjacents(&self) -> Vec<Coordinate> {
        let mut adjacents = Vec::new();

        adjacents.push(Coordinate {
            x: self.x + 1,
            y: self.y,
        });
        adjacents.push(Coordinate {
            x: self.x,
            y: self.y + 1,
        });
        if self.x > 0 {
            adjacents.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            })
        }
        if self.y > 0 {
            adjacents.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            })
        };

        adjacents
    }
}

#[derive(Debug, Clone, Copy)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug)]
struct Scanner {
    depth: u32,
    target: Coordinate,
    erosion_levels: Vec<Vec<u32>>,
}

impl Scanner {
    fn new(depth: u32, target: Coordinate) -> Scanner {
        let mut erosion_levels: Vec<Vec<u32>> = Vec::new();

        for y in 0..=(target.y + target.x + (7 * 2)) {
            let mut row = Vec::new();
            for x in 0..=(target.x + target.y + (7 * 2)) {
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

        Scanner {
            depth,
            target,
            erosion_levels,
        }
    }

    fn risk_level(&self) -> u32 {
        let mut risk_level = 0;

        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                risk_level += self.erosion_levels[y][x] % 3;
            }
        }

        risk_level
    }

    fn cave(&self) -> Cave {
        let mut regions = Vec::new();

        for y in 0..self.erosion_levels.len() {
            let mut row = Vec::new();
            for x in 0..self.erosion_levels[0].len() {
                row.push(match self.erosion_levels[y][x] % 3 {
                    0 => RegionType::Rocky,
                    1 => RegionType::Wet,
                    2 => RegionType::Narrow,
                    _ => panic!(),
                });
            }
            regions.push(row);
        }

        Cave { regions }
    }
}

#[derive(Debug)]
struct Cave {
    regions: Vec<Vec<RegionType>>,
}

impl Cave {
    #[allow(dead_code)]
    fn display(&self, target: &Coordinate) {
        for y in 0..self.regions.len() {
            for x in 0..self.regions[0].len() {
                if x == 0 && y == 0 {
                    print!("M");
                    continue;
                }
                if x == target.x && y == target.y {
                    print!("T");
                    continue;
                }
                print!(
                    "{}",
                    match self.regions[y][x] {
                        RegionType::Rocky => '.',
                        RegionType::Wet => '=',
                        RegionType::Narrow => '|',
                    }
                );
            }
            println!();
        }
    }

    fn shortest_path(&self, target: &Coordinate) -> u32 {
        use self::Equipment::*;
        use self::RegionType::*;

        let mut smallest_durations: HashMap<(Coordinate, Equipment), u32> = HashMap::new();
        let mut neighbors: BinaryHeap<Reverse<(u32, Coordinate, Equipment)>> = BinaryHeap::new();
        neighbors.push(Reverse((0, Coordinate { x: 0, y: 0 }, Torch)));

        while let Some(Reverse((duration, region, equipment))) = neighbors.pop() {
            if let Some(smallest_duration) = smallest_durations.get(&(region, equipment)) {
                if *smallest_duration <= duration {
                    continue;
                }
            }
            smallest_durations.insert((region, equipment), duration);

            if region == *target && equipment == Torch {
                return duration;
            }

            for adjacent in region.adjacents() {
                if match (self.regions[adjacent.y][adjacent.x], equipment) {
                    (Rocky, Torch) => true,
                    (Rocky, ClimbingGear) => true,
                    (Wet, ClimbingGear) => true,
                    (Wet, Neither) => true,
                    (Narrow, Torch) => true,
                    (Narrow, Neither) => true,
                    _ => false,
                } {
                    neighbors.push(Reverse((duration + 1, adjacent, equipment)));
                }
            }

            match (self.regions[region.y][region.x], equipment) {
                (Rocky, Torch) => neighbors.push(Reverse((duration + 7, region, ClimbingGear))),
                (Rocky, ClimbingGear) => neighbors.push(Reverse((duration + 7, region, Torch))),
                (Wet, ClimbingGear) => neighbors.push(Reverse((duration + 7, region, Neither))),
                (Wet, Neither) => neighbors.push(Reverse((duration + 7, region, ClimbingGear))),
                (Narrow, Torch) => neighbors.push(Reverse((duration + 7, region, Neither))),
                (Narrow, Neither) => neighbors.push(Reverse((duration + 7, region, Torch))),
                _ => (),
            }
        }

        u32::max_value()
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

fn scanner<'a>() -> Parser<'a, u8, Scanner> {
    (depth() + target()).map(|(depth, target)| Scanner::new(depth, target))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let scanner = scanner().parse(input.as_bytes())?;

    let cave = scanner.cave();
    //cave.display(&scanner.target);

    println!("Part 1: the total risk level is {}", scanner.risk_level());

    println!(
        "Part 2: the fewest minutes needed to reach the target is {}",
        cave.shortest_path(&scanner.target)
    );

    Ok(())
}
