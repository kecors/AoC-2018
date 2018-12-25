use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{stdin, Read};

#[derive(Debug, Eq, PartialEq)]
enum Terrain {
    Wall,
    Open,
}

#[derive(Debug, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.x.cmp(&other.x),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Allegiance {
    Elf,
    Goblin,
}

impl Allegiance {
    fn code(self) -> char {
        match self {
            Allegiance::Elf => 'E',
            Allegiance::Goblin => 'G',
        }
    }

    fn name(self) -> String {
        match self {
            Allegiance::Elf => String::from("Elves"),
            Allegiance::Goblin => String::from("Goblins"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Unit {
    allegiance: Allegiance,
    attack_power: u32,
    hit_points: u32,
}

#[derive(Debug)]
struct Engine {
    map: Vec<Vec<Terrain>>,
    unit_locations: HashMap<Location, Unit>,
    round_number: u32,
    targets_remain: bool,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut map = Vec::new();
        let mut unit_locations = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        row.push(Terrain::Wall);
                    }
                    '.' => {
                        row.push(Terrain::Open);
                    }
                    'E' => {
                        row.push(Terrain::Open);
                        unit_locations.insert(
                            Location { x, y },
                            Unit {
                                allegiance: Allegiance::Elf,
                                attack_power: 3,
                                hit_points: 200,
                            },
                        );
                    }
                    'G' => {
                        row.push(Terrain::Open);
                        unit_locations.insert(
                            Location { x, y },
                            Unit {
                                allegiance: Allegiance::Goblin,
                                attack_power: 3,
                                hit_points: 200,
                            },
                        );
                    }
                    _ => {
                        panic!("Unrecognized character in input");
                    }
                }
            }
            map.push(row);
        }

        Engine {
            map,
            unit_locations,
            round_number: 0,
            targets_remain: true,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for (y, row) in self.map.iter().enumerate() {
            let mut row_units = Vec::new();
            for (x, terrain) in row.iter().enumerate() {
                let location = Location { x, y };
                if let Some(unit) = self.unit_locations.get(&location) {
                    print!("{}", unit.allegiance.code());
                    row_units.push(unit);
                    continue;
                }

                match terrain {
                    Terrain::Wall => {
                        print!("#");
                    }
                    Terrain::Open => {
                        print!(".");
                    }
                }
            }
            for (n, unit) in row_units.iter().enumerate() {
                print!("{}", if n == 0 { "   " } else { ", " });
                print!("{}({})", unit.allegiance.code(), unit.hit_points);
            }
            println!();
        }
        println!();
    }

    // For a specified location, return a list of adjacent,
    // open locations (in reading order)
    fn adjacent_locations(&self, location: &Location) -> Vec<Location> {
        let mut adjacent_locations = Vec::new();

        if location.y != 0 && self.map[location.y - 1][location.x] == Terrain::Open {
            adjacent_locations.push(Location {
                x: location.x,
                y: location.y - 1,
            });
        }

        if location.x != 0 && self.map[location.y][location.x - 1] == Terrain::Open {
            adjacent_locations.push(Location {
                x: location.x - 1,
                y: location.y,
            });
        }

        if location.x < self.map[location.y].len()
            && self.map[location.y][location.x + 1] == Terrain::Open
        {
            adjacent_locations.push(Location {
                x: location.x + 1,
                y: location.y,
            });
        }

        if location.y < self.map.len() && self.map[location.y + 1][location.x] == Terrain::Open {
            adjacent_locations.push(Location {
                x: location.x,
                y: location.y + 1,
            });
        }

        adjacent_locations
    }

    // Return a list of all unit locations in reading order
    fn ordered_locations(&self) -> Vec<Location> {
        let mut locations: Vec<Location> = self.unit_locations.keys().cloned().collect();

        locations.sort();

        locations
    }

    // Return a list of locations at which target units can be found
    fn target_locations(&self, unit: &Unit) -> Vec<Location> {
        let target_locations: Vec<Location> = self
            .unit_locations
            .iter()
            .filter(|(_, target_unit)| target_unit.allegiance != unit.allegiance)
            .map(|(&target_location, _)| target_location)
            .collect();

        target_locations
    }

    // Return a list of open, unoccupied locations adjacent
    // to a target location
    fn in_range_locations(&self, target_locations: &[Location]) -> Vec<Location> {
        let mut in_range_locations_hs: HashSet<Location> = HashSet::new();

        for target_location in target_locations.iter() {
            for adjacent_location in self.adjacent_locations(target_location) {
                if self.unit_locations.get(&adjacent_location).is_none() {
                    in_range_locations_hs.insert(adjacent_location);
                }
            }
        }

        // Prepare to return the in range locations in a list in reading order
        let mut in_range_locations: Vec<Location> = in_range_locations_hs.drain().collect();
        in_range_locations.sort();

        in_range_locations
    }

    // For a specified location, return a hash map of
    // reachable locations and the minimum number of
    // steps required
    fn step_counts(&self, location: &Location) -> HashMap<Location, u32> {
        let mut step_counts: HashMap<Location, u32> = HashMap::new();

        let mut pending: HashSet<Location> = HashSet::new();
        pending.insert(*location);

        let mut steps = 0;

        loop {
            let current: Vec<Location> = pending.drain().collect();
            if current.is_empty() {
                break;
            }

            for stepped_to_location in current {
                // Cannot move through a unit to continue the search
                if self.unit_locations.contains_key(&stepped_to_location) {
                    continue;
                }

                // Store step count for a location only the
                // first time it is encountered. This will be
                // the minimum step count.
                step_counts.entry(stepped_to_location).or_insert(steps);

                // Process adjacent locations
                for adjacent_location in self.adjacent_locations(&stepped_to_location) {
                    if step_counts.contains_key(&adjacent_location) {
                        continue;
                    }
                    pending.insert(adjacent_location);
                }
            }
            steps += 1;
        }

        step_counts
    }

    // Return hash map of in range locations with their step counts
    fn reachable_locations(
        &self,
        in_range_locations: &[Location],
        step_counts: &HashMap<Location, u32>,
    ) -> HashMap<Location, u32> {
        let mut reachable_locations = HashMap::new();

        for location in in_range_locations.iter() {
            if let Some(steps) = step_counts.get(location) {
                reachable_locations.insert(*location, *steps);
            }
        }

        reachable_locations
    }

    // Return a list of locations reachable in the fewest steps
    fn nearest_locations(&self, reachable_locations: &HashMap<Location, u32>) -> Vec<Location> {
        let min_steps = reachable_locations
            .values()
            .min()
            .expect("Assume reachable_locations is non-empty");

        let mut nearest_locations: Vec<Location> = reachable_locations
            .iter()
            .filter(|(_, steps)| *steps == min_steps)
            .map(|(location, _)| *location)
            .collect();
        nearest_locations.sort();

        nearest_locations
    }

    fn move_unit(&mut self, location: Location, unit: &Unit) -> Location {
        let target_locations = self.target_locations(&unit);
        if target_locations.is_empty() {
            self.targets_remain = false;
            return location;
        }

        let in_range_locations: Vec<Location> = self.in_range_locations(&target_locations);
        if in_range_locations.is_empty() {
            return location;
        }

        let step_counts: HashMap<Location, u32> = self.step_counts(&location);
        if step_counts.is_empty() {
            return location;
        }

        let reachable_locations: HashMap<Location, u32> =
            self.reachable_locations(&in_range_locations, &step_counts);
        if reachable_locations.is_empty() {
            return location;
        }

        let nearest_locations = self.nearest_locations(&reachable_locations);
        let chosen_location = nearest_locations[0];

        if chosen_location == location {
            // No need to move
            return location;
        }

        // Now work backwards from the chosen location to the unit location

        let chosen_step_counts = self.step_counts(&chosen_location);
        if chosen_step_counts.is_empty() {
            return location;
        }
        let chosen_adjacent_locations = self.adjacent_locations(&location);
        if chosen_adjacent_locations.is_empty() {
            return location;
        }
        let chosen_reachable_locations =
            self.reachable_locations(&chosen_adjacent_locations, &chosen_step_counts);
        if chosen_reachable_locations.is_empty() {
            return location;
        }
        let chosen_nearest_locations = self.nearest_locations(&chosen_reachable_locations);
        if chosen_nearest_locations.is_empty() {
            return location;
        }

        chosen_nearest_locations[0]
    }

    fn attack(&mut self, attacking_location: Location, attacking_unit: &Unit) {
        // Select target from adjacent targets
        let mut adjacent_targets: Vec<(Location, u32)> = Vec::new();
        for adjacent_location in self.adjacent_locations(&attacking_location) {
            if let Some(adjacent_unit) = self.unit_locations.get(&adjacent_location) {
                if adjacent_unit.allegiance != attacking_unit.allegiance {
                    adjacent_targets.push((adjacent_location, adjacent_unit.hit_points));
                }
            }
        }

        // Return if there are no adjacent targets
        if adjacent_targets.is_empty() {
            return;
        }

        // Select the target with the fewest hit points; tie breaking
        // by reading order was provided by fn adjacent_locations()
        let (target_location, _target_hit_points) =
            adjacent_targets.iter().min_by_key(|x| x.1).unwrap();

        // Attack
        let mut dies_flag = false;
        if let Some(attacked_unit) = self.unit_locations.get_mut(&target_location) {
            if attacked_unit.hit_points > attacking_unit.attack_power {
                attacked_unit.hit_points -= attacking_unit.attack_power;
            } else {
                dies_flag = true;
            }
        }
        if dies_flag {
            self.unit_locations.remove(&target_location);
        }
    }

    fn round(&mut self) -> bool {
        for location in self.ordered_locations() {
            let unit = if let Some(unit) = self.unit_locations.remove(&location) {
                unit
            } else {
                // This unit must have been killed earler in the round
                continue;
            };

            let new_location = self.move_unit(location, &unit);
            self.attack(new_location, &unit);

            self.unit_locations.insert(new_location, unit);
        }

        if self.targets_remain {
            self.round_number += 1;
        }

        self.targets_remain
    }

    fn hit_point_totals(&self) -> HashMap<Allegiance, u32> {
        let mut totals = HashMap::new();

        for unit in self.unit_locations.values() {
            *totals.entry(unit.allegiance).or_insert(0) += unit.hit_points;
        }

        totals
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut engine = Engine::new(input.trim());
    engine.display();

    while engine.round() {
        println!("After {} rounds:", engine.round_number);
        engine.display();
    }
    println!("Final:");
    engine.display();

    println!(
        "Part 1: Combat ends after {} full rounds",
        engine.round_number
    );
    for (allegiance, hit_point_total) in engine.hit_point_totals() {
        println!(
            "{} win with {} total hit points left",
            allegiance.name(),
            hit_point_total
        );
        println!(
            "Outcome: {} * {} = {}",
            engine.round_number,
            hit_point_total,
            engine.round_number * hit_point_total
        );
    }

    // Part 2

    let mut elf_attack_power = 3;

    loop {
        elf_attack_power += 1;
        println!("Elf attack power is {}", elf_attack_power);

        let mut engine = Engine::new(input.trim());
        let mut starting_elf_count = 0;

        for unit in engine.unit_locations.values_mut() {
            if unit.allegiance == Allegiance::Elf {
                unit.attack_power = elf_attack_power;
                starting_elf_count += 1;
            }
        }

        while engine.round() {}

        let remaining_elves: Vec<&Unit> = engine
            .unit_locations
            .values()
            .filter(|x| x.allegiance == Allegiance::Elf)
            .collect();
        if remaining_elves.len() < starting_elf_count {
            continue;
        }

        println!(
            "Part 2: Combat ends after {} full rounds",
            engine.round_number
        );
        for (allegiance, hit_point_total) in engine.hit_point_totals() {
            if allegiance == Allegiance::Elf {
                println!(
                    "{} win with {} total hit points left",
                    allegiance.name(),
                    hit_point_total
                );
                println!(
                    "Outcome: {} * {} = {}",
                    engine.round_number,
                    hit_point_total,
                    engine.round_number * hit_point_total
                );

                return;
            }
        }
    }
}
