use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Location {
    x: u32,
    y: u32,
}

fn solve(coordinates: &[Location]) -> u32 {
    let min_x = coordinates.iter().fold(0, |acc, c| c.x.min(acc));
    let min_y = coordinates.iter().fold(0, |acc, c| c.y.min(acc));
    let max_x = coordinates.iter().fold(0, |acc, c| c.x.max(acc));
    let max_y = coordinates.iter().fold(0, |acc, c| c.y.max(acc));

    // Determine the closest coordinate for each location
    let mut coordinate_locations: HashMap<Location, Vec<Location>> = HashMap::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // cds = coordinate distances (from location)
            let mut cds_hm = HashMap::new();
            for &coordinate in coordinates.iter() {
                let distance =
                    (coordinate.x as i32 - x as i32).abs() + (coordinate.y as i32 - y as i32).abs();
                cds_hm.insert(coordinate, distance);
            }
            let mut cds_vec: Vec<(Location, i32)> = cds_hm.drain().collect();
            cds_vec.sort_by(|a, b| a.1.cmp(&b.1));
            if cds_vec[0].1 < cds_vec[1].1 {
                let cl = coordinate_locations
                    .entry(cds_vec[0].0)
                    .or_insert_with(Vec::new);
                cl.push(Location { x, y });
            }
        }
    }

    let mut locations_max = (0, None);
    for (coordinate, locations) in coordinate_locations.iter() {
        // Remove from consideration any coordinates with a location on an edge
        if locations
            .iter()
            .any(|l| l.x == min_x || l.x == max_x || l.y == min_y || l.y == max_y)
        {
            continue;
        }
        if locations.len() > locations_max.0 {
            locations_max = (locations.len(), Some(coordinate));
        }
    }

    locations_max.0 as u32
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let coordinates = input.lines().fold(Vec::new(), |mut acc, line| {
        let mut fields = line.trim().split(", ");
        let x = fields.next().unwrap().parse().unwrap();
        let y = fields.next().unwrap().parse().unwrap();
        acc.push(Location { x, y });
        acc
    });

    let part1 = solve(&coordinates);
    println!(
        "Part 1: the size of the largest area that isn't infinite is {}",
        part1
    );
}
