use std::cmp::Ordering;
use std::collections::HashMap;
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
    facing: Facing,
    next_turn: Turn,
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<Terrain>>,
    cart_locations: HashMap<Location, Cart>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut grid = Vec::new();
        let mut cart_locations = HashMap::new();

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
                        cart_locations.insert(
                            Location { x, y },
                            Cart {
                                facing: Facing::North,
                                next_turn: Turn::Left,
                            },
                        );
                    }
                    '>' => {
                        row.push(Terrain::Horizontal);
                        cart_locations.insert(
                            Location { x, y },
                            Cart {
                                facing: Facing::East,
                                next_turn: Turn::Left,
                            },
                        );
                    }
                    'v' => {
                        row.push(Terrain::Vertical);
                        cart_locations.insert(
                            Location { x, y },
                            Cart {
                                facing: Facing::South,
                                next_turn: Turn::Left,
                            },
                        );
                    }
                    '<' => {
                        row.push(Terrain::Horizontal);
                        cart_locations.insert(
                            Location { x, y },
                            Cart {
                                facing: Facing::West,
                                next_turn: Turn::Left,
                            },
                        );
                    }
                    _ => panic!("Unexpected character in input"),
                };
            }
            grid.push(row);
        }

        Engine {
            grid,
            cart_locations,
        }
    }

    fn tick(&mut self, part1_flag: bool) -> Option<Location> {
        let mut cart_location_keys: Vec<Location> = Vec::new();
        for location in self.cart_locations.keys() {
            cart_location_keys.push(*location);
        }
        cart_location_keys.sort_by(|a, b| match a.y.cmp(&b.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.x.cmp(&b.x),
            Ordering::Greater => Ordering::Greater,
        });

        for mut location in cart_location_keys {
            if let Some(mut cart) = self.cart_locations.remove(&location) {
                // Move cart
                match cart.facing {
                    Facing::North => {
                        location.y -= 1;
                    }
                    Facing::East => {
                        location.x += 1;
                    }
                    Facing::South => {
                        location.y += 1;
                    }
                    Facing::West => {
                        location.x -= 1;
                    }
                }

                // If a crash has occurred, remove the other cart and proceed
                // 
                // Note that this logic does not handle the scenario where 
                // three carts crash at once.
                if let Some(_cart) = self.cart_locations.remove(&location) {
                    if part1_flag {
                        return Some(location);
                    } else {
                        continue;
                    }
                }

                // Turn cart, if appropriate
                match self.grid[location.y][location.x] {
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

                // Store the cart at its new location
                self.cart_locations.insert(location, cart);
            }
        }

        // Return
        match self.cart_locations.len() {
            0 => {
                panic!("No carts remain!");
            }
            1 => {
                let (location, _cart) = self.cart_locations.drain().next().unwrap();
                Some(location)
            }
            _ => {
                None
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input);
    let location = loop {
        if let Some(location) = engine.tick(true) {
            break location;
        }
    };
    println!(
        "Part 1: the location of the first crash is {},{}",
        location.x, location.y
    );

    let mut engine = Engine::new(&input);
    let location = loop {
        if let Some(location) = engine.tick(false) {
            break location;
        }
    };
    println!(
        "Part 2: the location of the last cart is {},{}",
        location.x, location.y
    );
}
