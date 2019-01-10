use pom::parser::*;
use pom::Error;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Bounds {
    fn new() -> Bounds {
        let min_x = usize::max_value();
        let max_x = 0;
        let min_y = usize::max_value();
        let max_y = 0;

        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

#[derive(Debug)]
enum Vein {
    Horizontal { y: usize, x0: usize, x1: usize },
    Vertical { x: usize, y0: usize, y1: usize },
}

impl Vein {
    fn bound(&self, bounds: &mut Bounds) {
        match self {
            Vein::Horizontal { y, x0, x1 } => {
                if *y < bounds.min_y {
                    bounds.min_y = *y;
                }
                if *y > bounds.max_y {
                    bounds.max_y = *y;
                }
                if *x0 < bounds.min_x {
                    bounds.min_x = *x0;
                }
                if *x1 > bounds.max_x {
                    bounds.max_x = *x1;
                }
            }
            Vein::Vertical { x, y0, y1 } => {
                if *x < bounds.min_x {
                    bounds.min_x = *x;
                }
                if *x > bounds.max_x {
                    bounds.max_x = *x;
                }
                if *y0 < bounds.min_y {
                    bounds.min_y = *y0;
                }
                if *y1 > bounds.max_y {
                    bounds.max_y = *y1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Material {
    Sand,
    Clay,
    Well,
    FlowingWater,
    SettledWater,
}

#[derive(Debug)]
struct Engine {
    bounds: Bounds,
    squares: Vec<Vec<Material>>,
}

impl Engine {
    fn new(veins: &[Vein]) -> Engine {
        let bounds = veins.iter().fold(Bounds::new(), |mut bounds, vein| {
            vein.bound(&mut bounds);
            bounds
        });

        let width = bounds.max_x - bounds.min_x + 1 + 2;
        let height = bounds.max_y + 1;
        let mut squares = vec![vec![Material::Sand; width]; height];

        let well_x = 500 - bounds.min_x + 1;
        squares[0][well_x] = Material::Well;

        for vein in veins.iter() {
            match vein {
                Vein::Horizontal { y, x0, x1 } => {
                    for x in *x0..=*x1 {
                        squares[*y][x - bounds.min_x + 1] = Material::Clay;
                    }
                }
                Vein::Vertical { x, y0, y1 } => {
                    for y in *y0..=*y1 {
                        squares[y][*x - bounds.min_x + 1] = Material::Clay;
                    }
                }
            }
        }

        Engine { bounds, squares }
    }

    fn display(&self) {
        for y in 0..self.squares.len() {
            for x in 0..self.squares[0].len() {
                print!(
                    "{}",
                    match self.squares[y][x] {
                        Material::Sand => '.',
                        Material::Clay => '#',
                        Material::Well => '+',
                        Material::FlowingWater => '|',
                        Material::SettledWater => '~',
                    }
                );
            }
            println!();
        }
        println!();
    }

    fn flow(&mut self, x: usize, y: usize) {
        use self::Material::*;

        if y + 1 >= self.squares.len() {
            self.squares[y][x] = FlowingWater;
            return;
        }

        if self.squares[y + 1][x] == Sand {
            self.flow(x, y + 1);
        }

        if self.squares[y + 1][x] == FlowingWater {
            self.squares[y][x] = FlowingWater;
            return;
        }

        // Flow to the left
        let mut left_offset = 0;
        let mut left_anchor = None;

        loop {
            match self.squares[y][x - left_offset] {
                Clay => {
                    left_anchor = Some(x - left_offset + 1);
                    break;
                }
                Sand | FlowingWater => match self.squares[y + 1][x - left_offset] {
                    Clay | SettledWater => {
                        left_offset += 1;
                        continue;
                    }
                    Sand | FlowingWater => {
                        for x_ in (x - left_offset)..=x {
                            self.squares[y][x_] = FlowingWater;
                        }
                        self.flow(x - left_offset, y);
                        break;
                    }
                    _ => {
                        panic!(
                            "x {} y {} material {:?}",
                            x - left_offset,
                            y + 1,
                            self.squares[y + 1][x - left_offset]
                        );
                    }
                },
                _ => {
                    panic!(
                        "x {} y {} material {:?}",
                        x - left_offset,
                        y,
                        self.squares[y][x - left_offset]
                    );
                }
            }
        }

        // Flow to the right
        let mut right_offset = 0;

        loop {
            match self.squares[y][x + right_offset] {
                Clay | SettledWater => {
                    if let Some(left_x) = left_anchor {
                        for x_ in left_x..(x + right_offset) {
                            self.squares[y][x_] = SettledWater;
                        }
                        return;
                    } else {
                        for x_ in x..(x + right_offset) {
                            self.squares[y][x_] = FlowingWater;
                        }
                        return;
                    }
                }
                Sand | FlowingWater => match self.squares[y + 1][x + right_offset] {
                    Clay | SettledWater => {
                        right_offset += 1;
                        continue;
                    }
                    Sand | FlowingWater => {
                        let left_x = if let Some(a) = left_anchor { a } else { x };
                        for x_ in left_x..=(x + right_offset) {
                            self.squares[y][x_] = FlowingWater;
                        }
                        self.flow(x + right_offset, y);
                        return;
                    }
                    _ => {
                        panic!(
                            "x {} y {} material {:?}",
                            x + right_offset,
                            y + 1,
                            self.squares[y + 1][x + right_offset]
                        );
                    }
                },
                _ => {
                    panic!(
                        "x {} y {} material {:?}",
                        x + right_offset,
                        y,
                        self.squares[y][x + right_offset]
                    );
                }
            }
        }
    }

    fn water_tile_count(&self) -> (u32, u32) {
        let flowing: usize = self
            .squares
            .iter()
            .enumerate()
            .filter(|(y, _)| *y >= self.bounds.min_y && *y <= self.bounds.max_y)
            .map(|(_, row)| row.iter().filter(|&&x| x == Material::FlowingWater).count())
            .sum();

        let settled: usize = self
            .squares
            .iter()
            .flatten()
            .filter(|&&x| x == Material::SettledWater)
            .count();

        (flowing as u32, settled as u32)
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

fn horizontal<'a>() -> Parser<'a, u8, Vein> {
    (space() * (seq(b"y=") * number())
        + ((sym(b',') * space() * seq(b"x=") * number()) + (seq(b"..") * number())))
    .map(|(y, (x0, x1))| Vein::Horizontal { y, x0, x1 })
}

fn vertical<'a>() -> Parser<'a, u8, Vein> {
    (space() * (seq(b"x=") * number())
        + ((sym(b',') * space() * seq(b"y=") * number()) + (seq(b"..") * number())))
    .map(|(x, (y0, y1))| Vein::Vertical { x, y0, y1 })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (horizontal() | vertical())
        .repeat(1..)
        .map(|veins| Engine::new(&veins))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;
    engine.flow(500 - engine.bounds.min_x + 1, 1);
    engine.display();

    let (flowing, settled) = engine.water_tile_count();
    println!("Part 1: the water can reach {} tiles", flowing + settled);
    println!("Part 2: {} tiles are left", settled);

    Ok(())
}
