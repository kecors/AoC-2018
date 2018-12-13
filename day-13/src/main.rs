use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Debug)]
enum Terrain {
    Unpassable,
    Horizontal,
    Vertical,
    Intersection,
    CurveLeft,
    CurveRight,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    location: Location,
    facing: Facing,
    next_turn: Turn,
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<Terrain>>,
    carts: Vec<Cart>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut grid = Vec::new();
        let mut carts = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    ' ' => {
                        row.push(Terrain::Unpassable);
                    }
                    '-' => {
                        row.push(Terrain::Horizontal);
                    }
                    '|' => {
                        row.push(Terrain::Vertical);
                    }
                    '+' => {
                        row.push(Terrain::Intersection);
                    }
                    '\\' => {
                        row.push(Terrain::CurveLeft);
                    }
                    '/' => {
                        row.push(Terrain::CurveRight);
                    }
                    '^' => {
                        row.push(Terrain::Vertical);
                        carts.push(Cart {
                            location: Location { x, y },
                            facing: Facing::North,
                            next_turn: Turn::Left,
                        });
                    }
                    '>' => {
                        row.push(Terrain::Horizontal);
                        carts.push(Cart {
                            location: Location { x, y },
                            facing: Facing::East,
                            next_turn: Turn::Left,
                        });
                    }
                    'v' => {
                        row.push(Terrain::Vertical);
                        carts.push(Cart {
                            location: Location { x, y },
                            facing: Facing::South,
                            next_turn: Turn::Left,
                        });
                    }
                    '<' => {
                        row.push(Terrain::Horizontal);
                        carts.push(Cart {
                            location: Location { x, y },
                            facing: Facing::West,
                            next_turn: Turn::Left,
                        });
                    }
                    _ => panic!("Unexpected character in input"),
                };
            }
            grid.push(row);
        }

        Engine { grid, carts }
    }

    fn tick(&mut self) -> Option<Location> {
        for cart in self.carts.iter_mut() {
            // Move cart
            match cart.facing {
                Facing::North => {
                    cart.location.y -= 1;
                }
                Facing::East => {
                    cart.location.x += 1;
                }
                Facing::South => {
                    cart.location.y += 1;
                }
                Facing::West => {
                    cart.location.x -= 1;
                }
            }

            // Turn cart, if appropriate
            match self.grid[cart.location.y][cart.location.x] {
                Terrain::Intersection => match cart.facing {
                    Facing::North => match cart.next_turn {
                        Turn::Left => {
                            cart.facing = Facing::West;
                            cart.next_turn = Turn::Straight;
                        }
                        Turn::Straight => {
                            cart.next_turn = Turn::Right;
                        }
                        Turn::Right => {
                            cart.facing = Facing::East;
                            cart.next_turn = Turn::Left;
                        }
                    },
                    Facing::East => match cart.next_turn {
                        Turn::Left => {
                            cart.facing = Facing::North;
                            cart.next_turn = Turn::Straight;
                        }
                        Turn::Straight => {
                            cart.next_turn = Turn::Right;
                        }
                        Turn::Right => {
                            cart.facing = Facing::South;
                            cart.next_turn = Turn::Left;
                        }
                    },
                    Facing::South => match cart.next_turn {
                        Turn::Left => {
                            cart.facing = Facing::East;
                            cart.next_turn = Turn::Straight;
                        }
                        Turn::Straight => {
                            cart.next_turn = Turn::Right;
                        }
                        Turn::Right => {
                            cart.facing = Facing::West;
                            cart.next_turn = Turn::Left;
                        }
                    },
                    Facing::West => match cart.next_turn {
                        Turn::Left => {
                            cart.facing = Facing::South;
                            cart.next_turn = Turn::Straight;
                        }
                        Turn::Straight => {
                            cart.next_turn = Turn::Right;
                        }
                        Turn::Right => {
                            cart.facing = Facing::North;
                            cart.next_turn = Turn::Left;
                        }
                    },
                },
                Terrain::CurveLeft => match cart.facing {
                    Facing::North => {
                        cart.facing = Facing::West;
                    }
                    Facing::East => {
                        cart.facing = Facing::South;
                    }
                    Facing::South => {
                        cart.facing = Facing::East;
                    }
                    Facing::West => {
                        cart.facing = Facing::North;
                    }
                },
                Terrain::CurveRight => match cart.facing {
                    Facing::North => {
                        cart.facing = Facing::East;
                    }
                    Facing::East => {
                        cart.facing = Facing::North;
                    }
                    Facing::South => {
                        cart.facing = Facing::West;
                    }
                    Facing::West => {
                        cart.facing = Facing::South;
                    }
                },
                _ => {}
            }
        }

        // Report crashes
        let mut location_hs = HashSet::new();
        for cart in self.carts.iter() {
            if !location_hs.insert(cart.location) {
                return Some(cart.location);
            }
        }

        None
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input);

    let location = loop {
        if let Some(location) = engine.tick() {
            break location;
        }
    };
    println!(
        "Part 1: the location of the first crash is {},{}",
        location.x, location.y
    );
}
