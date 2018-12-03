use std::collections::HashSet;
use std::io::{stdin, Read};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "claim.pest"]
struct ClaimParser;

#[derive(Debug, Default)]
struct Claim {
    id: u32,
    left_offset: u32,
    top_offset: u32,
    width: u32,
    height: u32,
}

fn parse(line: &str) -> Claim {
    let pairs = ClaimParser::parse(Rule::claim, line).unwrap_or_else(|e| panic!("{}", e));

    let mut claim = Claim::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::id => {
                claim.id = pair.as_str().parse().unwrap();
            }
            Rule::left_offset => {
                claim.left_offset = pair.as_str().parse().unwrap();
            }
            Rule::top_offset => {
                claim.top_offset = pair.as_str().parse().unwrap();
            }
            Rule::width => {
                claim.width = pair.as_str().parse().unwrap();
            }
            Rule::height => {
                claim.height = pair.as_str().parse().unwrap();
            }
            _ => {}
        }
    }

    claim
}

fn solve(claims: &[Claim]) -> (u32, u32) {
    let mut max_x = 1000;
    let mut max_y = 1000;

    for claim in claims.iter() {
        if claim.left_offset + claim.width > max_x {
            max_x = claim.left_offset + claim.width;
        }
        if claim.top_offset + claim.height > max_y {
            max_y = claim.top_offset + claim.height;
        }
    }

    let mut square_claim_ids = vec![vec![vec![]; max_x as usize]; max_y as usize];

    for claim in claims.iter() {
        for dy in 0..claim.height {
            for dx in 0..claim.width {
                let x = (claim.left_offset + dx) as usize;
                let y = (claim.top_offset + dy) as usize;
                square_claim_ids[x][y].push(claim.id);
            }
        }
    }

    let mut claim_multiples = 0;
    let mut claim_id_hs: HashSet<u32> = claims.iter().map(|x| x.id).collect();

    for y in 0..(max_y as usize) {
        for x in 0..(max_x as usize) {
            if square_claim_ids[x][y].len() >= 2 {
                claim_multiples += 1;
                for claim_id in square_claim_ids[x][y].iter() {
                    claim_id_hs.remove(&claim_id);
                }
            }
        }
    }

    if claim_id_hs.len() != 1 {
        panic!("The problem description states that exactly one claim does not overlap");
    }

    let non_overlapped_claim = claim_id_hs.drain().next().unwrap();

    (claim_multiples, non_overlapped_claim)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let claims: Vec<Claim> = input.lines().map(|x| parse(x)).collect();

    let (part1, part2) = solve(&claims);
    println!(
        "Part 1: {} square inches of fabric are within two or more claims",
        part1
    );
    println!("Part 2: the only claim that doesn't overlap is {}", part2);
}
