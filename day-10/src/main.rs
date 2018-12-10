use regex::Regex;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Debug)]
struct Sky {
    points: Vec<Point>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    width: i32,
    height: i32,
}

impl Sky {
    fn new(points: Vec<Point>) -> Sky {
        Sky {
            points,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            width: 0,
            height: 0,
        }
    }

    fn tick(&mut self) {
        self.min_x = i32::max_value();
        self.max_x = i32::min_value();
        self.min_y = i32::max_value();
        self.max_y = i32::min_value();
        for point in self.points.iter_mut() {
            point.x += point.dx;
            point.y += point.dy;
            if point.x < self.min_x {
                self.min_x = point.x;
            }
            if point.x > self.max_x {
                self.max_x = point.x;
            }
            if point.y < self.min_y {
                self.min_y = point.y;
            }
            if point.y > self.max_y {
                self.max_y = point.y;
            }
        }
        self.width = (self.max_x - self.min_x).abs();
        self.height = (self.max_y - self.min_y).abs();
    }

    fn display(&self) {
        let mut area = vec![vec![false; self.max_x as usize + 1]; self.max_y as usize + 1];

        for point in self.points.iter() {
            area[point.y as usize][point.x as usize] = true;
        }

        for y in 0..self.max_y as usize + 1 {
            for x in 0..self.max_x as usize + 1 {
                print!("{}", if area[y][x] { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let rule = r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$";
    let re = Regex::new(rule).unwrap();

    let mut points = Vec::new();
    for line in input.trim().lines() {
        if let Some(captures) = re.captures(line) {
            let x: i32 = captures[1].parse().unwrap();
            let y: i32 = captures[2].parse().unwrap();
            let dx: i32 = captures[3].parse().unwrap();
            let dy: i32 = captures[4].parse().unwrap();
            points.push(Point { x, y, dx, dy });
        }
    }

    let mut sky = Sky::new(points);

    for j in 1..10500 {
        sky.tick();
        if sky.min_x >= 0 && sky.min_y >= 0 {
            println!("j = {}", j);
            sky.display();
        }
    }
}
