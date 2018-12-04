use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{stdin, Read};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "event.pest"]
struct EventParser;

#[derive(Debug, Default)]
struct Timestamp {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
enum Action {
    BeginsShift { guard_id: u32 },
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Event {
    timestamp: Timestamp,
    action: Action,
}

fn parse(line: &str) -> Event {
    let pairs = EventParser::parse(Rule::event, line).unwrap_or_else(|e| panic!("{}", e));

    let mut timestamp = Timestamp::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::timestamp => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::year => {
                            timestamp.year = inner_pair.as_str().parse().unwrap();
                        }
                        Rule::month => {
                            timestamp.month = inner_pair.as_str().parse().unwrap();
                        }
                        Rule::day => {
                            timestamp.day = inner_pair.as_str().parse().unwrap();
                        }
                        Rule::hour => {
                            timestamp.hour = inner_pair.as_str().parse().unwrap();
                        }
                        Rule::minute => {
                            timestamp.minute = inner_pair.as_str().parse().unwrap();
                        }
                        _ => {}
                    }
                }
            }
            Rule::begins_shift => {
                let guard_id = pair.into_inner().next().unwrap().as_str().parse().unwrap();
                return Event {
                    timestamp,
                    action: Action::BeginsShift { guard_id },
                };
            }
            Rule::falls_asleep => {
                return Event {
                    timestamp,
                    action: Action::FallsAsleep,
                };
            }
            Rule::wakes_up => {
                return Event {
                    timestamp,
                    action: Action::WakesUp,
                };
            }
            _ => {}
        }
    }
    panic!("This function should always return after an action is parsed");
}

#[derive(Debug, Default)]
struct Shift {
    timestamp: Timestamp,
    guard_id: u32,
    sleeping_minutes: Vec<bool>,
}

impl Shift {
    fn new(timestamp: Timestamp, guard_id: u32) -> Shift {
        let sleeping_minutes = vec![false; 60];
        Shift {
            timestamp,
            guard_id,
            sleeping_minutes,
        }
    }
}

fn solve(mut events: Vec<Event>) -> (u32, u32) {
    events.sort_by(|a, b| {
        if a.timestamp.year > b.timestamp.year {
            return Ordering::Greater;
        }
        if a.timestamp.year < b.timestamp.year {
            return Ordering::Less;
        }
        if a.timestamp.month > b.timestamp.month {
            return Ordering::Greater;
        }
        if a.timestamp.month < b.timestamp.month {
            return Ordering::Less;
        }
        if a.timestamp.day > b.timestamp.day {
            return Ordering::Greater;
        }
        if a.timestamp.day < b.timestamp.day {
            return Ordering::Less;
        }
        if a.timestamp.hour > b.timestamp.hour {
            return Ordering::Greater;
        }
        if a.timestamp.hour < b.timestamp.hour {
            return Ordering::Less;
        }
        if a.timestamp.minute > b.timestamp.minute {
            return Ordering::Greater;
        }
        if a.timestamp.minute < b.timestamp.minute {
            return Ordering::Less;
        }
        Ordering::Equal
    });

    let mut shifts = Vec::new();
    let mut first_pass_flag = true;
    let mut shift = Shift::default(); // Value will be discarded
    let mut falls_asleep_minute = 0; // Value will be discarded
    let mut guard_sleep_minutes: HashMap<u32, u32> = HashMap::new();

    for event in events {
        match event.action {
            Action::BeginsShift { guard_id } => {
                if first_pass_flag {
                    first_pass_flag = false;
                } else {
                    shifts.push(shift);
                }
                shift = Shift::new(event.timestamp, guard_id);
            }
            Action::FallsAsleep => {
                falls_asleep_minute = event.timestamp.minute;
            }
            Action::WakesUp => {
                let wakes_up_minute = event.timestamp.minute;
                for j in falls_asleep_minute..wakes_up_minute {
                    shift.sleeping_minutes[j as usize] = true;
                }
                *guard_sleep_minutes.entry(shift.guard_id).or_insert(0) +=
                    u32::from(wakes_up_minute - falls_asleep_minute);
            }
        }
    }

    let mut max_minutes = 0;
    let mut max_minutes_guard_id = 0; // Value will be discarded
    for (guard_id, minutes) in guard_sleep_minutes.drain() {
        if minutes > max_minutes {
            max_minutes = minutes;
            max_minutes_guard_id = guard_id;
        }
    }

    let mut minute_sleep_frequency = HashMap::new();

    for shift in shifts.iter() {
        for minute in 0..60 {
            if shift.sleeping_minutes[minute] {
                let frequency = minute_sleep_frequency
                    .entry(shift.guard_id)
                    .or_insert(vec![0; 60]);
                frequency[minute] += 1;
            }
        }
    }

    let mut part1 = None;
    let mut all_max_frequency = 0;
    let mut all_max_frequency_minute = 0;
    let mut all_max_frequency_guard_id = 0;

    for (&guard_id, frequency) in minute_sleep_frequency.iter() {
        let mut guard_max_frequency = 0;
        let mut guard_max_frequency_minute = 0; // Value will be discarded
        for minute in 0..60 {
            if frequency[minute] > guard_max_frequency {
                guard_max_frequency = frequency[minute];
                guard_max_frequency_minute = minute;
            }
        }
        if guard_id == max_minutes_guard_id {
            part1 = Some(max_minutes_guard_id * guard_max_frequency_minute as u32);
        }
        if guard_max_frequency > all_max_frequency {
            all_max_frequency = guard_max_frequency;
            all_max_frequency_minute = guard_max_frequency_minute;
            all_max_frequency_guard_id = guard_id;
        }
    }

    let part2 = all_max_frequency_guard_id * all_max_frequency_minute as u32;

    (part1.unwrap(), part2)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let events: Vec<Event> = input.lines().map(|x| parse(x)).collect();

    let (part1, part2) = solve(events);
    println!(
        "Part 1: the product of the chosen guard ID and the minute is {}",
        part1
    );
    println!(
        "Part 2: the product of the chosen guard ID and the minute is {}",
        part2
    );
}
