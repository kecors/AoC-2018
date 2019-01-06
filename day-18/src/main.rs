use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Terrain {
    Open,
    Tree,
    Yard,
}

fn terrain_counts(terrains: &[Terrain]) -> (u32, u32) {
    let trees = terrains.iter().filter(|&&x| x == Terrain::Tree).count() as u32;

    let yards = terrains.iter().filter(|&&x| x == Terrain::Yard).count() as u32;

    (trees, yards)
}

#[derive(Debug)]
struct Engine {
    area: Vec<Vec<Terrain>>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut area: Vec<Vec<Terrain>> = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(match ch {
                    '.' => Terrain::Open,
                    '|' => Terrain::Tree,
                    '#' => Terrain::Yard,
                    _ => {
                        panic!("Unexpected input");
                    }
                });
            }
            area.push(row);
        }

        Engine { area }
    }

    fn display(&self, minute: u32) {
        println!("minute {}", minute);
        for y in 0..self.area.len() {
            for x in 0..self.area[0].len() {
                print!(
                    "{}",
                    match self.area[y][x] {
                        Terrain::Open => ".",
                        Terrain::Tree => "|",
                        Terrain::Yard => "#",
                    }
                );
            }
            println!();
        }
        println!();
    }

    fn tick(&mut self) {
        let mut area: Vec<Vec<Terrain>> = Vec::new();

        for y in 0..self.area.len() {
            let mut row = Vec::new();
            for x in 0..self.area[0].len() {
                let adjacent_terrains: Vec<Terrain> = self
                    .adjacents(x, y)
                    .iter()
                    .map(|(x, y)| self.area[*y][*x])
                    .collect();
                let (trees, yards) = terrain_counts(&adjacent_terrains);
                let terrain = match self.area[y][x] {
                    Terrain::Open => {
                        if trees >= 3 {
                            Terrain::Tree
                        } else {
                            Terrain::Open
                        }
                    }
                    Terrain::Tree => {
                        if yards >= 3 {
                            Terrain::Yard
                        } else {
                            Terrain::Tree
                        }
                    }
                    Terrain::Yard => {
                        if trees >= 1 && yards >= 1 {
                            Terrain::Yard
                        } else {
                            Terrain::Open
                        }
                    }
                };
                row.push(terrain);
            }
            area.push(row);
        }

        self.area = area;
    }

    fn adjacents(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let left_edge: bool = x == 0;
        let right_edge: bool = x == self.area[0].len() - 1;
        let top_edge: bool = y == 0;
        let bottom_edge: bool = y == self.area.len() - 1;

        let mut adjacents = Vec::new();

        if !left_edge && !top_edge {
            adjacents.push((x - 1, y - 1))
        }
        if !top_edge {
            adjacents.push((x, y - 1))
        }
        if !right_edge && !top_edge {
            adjacents.push((x + 1, y - 1))
        }
        if !left_edge {
            adjacents.push((x - 1, y))
        }
        if !right_edge {
            adjacents.push((x + 1, y))
        }
        if !left_edge && !bottom_edge {
            adjacents.push((x - 1, y + 1))
        }
        if !bottom_edge {
            adjacents.push((x, y + 1))
        }
        if !bottom_edge && !right_edge {
            adjacents.push((x + 1, y + 1))
        }

        adjacents
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let mut engine = Engine::new(&input);

    engine.display(0);
    for minute in 1..=10 {
        engine.tick();
        engine.display(minute);
    }
    let terrains: Vec<Terrain> = engine.area.iter().cloned().flatten().collect();
    let (trees, yards) = terrain_counts(&terrains);
    println!(
        "Part 1: the total resource value of the area after 10 minutes is {}",
        trees * yards
    );

    // Part 2
    let mut engine = Engine::new(&input);

    let mut areas: HashMap<Vec<Vec<Terrain>>, u32> = HashMap::new();
    let mut minute = 0;

    let cycle_start_minute = loop {
        //engine.display(minute);
        match areas.entry(engine.area.clone()) {
            Entry::Vacant(v) => {
                v.insert(minute);
            }
            Entry::Occupied(o) => {
                break *o.get();
            }
        }

        engine.tick();
        minute += 1;
    };

    let cycle_length = minute - cycle_start_minute;
    let solution_minute =
        ((1_000_000_000 - cycle_start_minute) % cycle_length) + cycle_start_minute;
    areas
        .iter()
        .filter(|(_, minute)| **minute == solution_minute)
        .for_each(|(area, _)| {
            let terrains: Vec<Terrain> = area.iter().cloned().flatten().collect();
            let (trees, yards) = terrain_counts(&terrains);
            println!(
                "Part 2: the total resource value after 1000000000 minutes is {}",
                trees * yards
            );
        });
}
