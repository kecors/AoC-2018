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

fn solve_part1(claims: &[Claim]) -> u32 {
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

    let mut claim_counts = vec![vec![0; max_x as usize]; max_y as usize];

    for claim in claims.iter() {
        for dy in 0..claim.height {
            for dx in 0..claim.width {
                let x = (claim.left_offset + dx) as usize;
                let y = (claim.top_offset + dy) as usize;
                claim_counts[x][y] += 1;
            }
        }
    }

    let mut claim_multiples = 0;

    for y in 0..(max_y as usize) {
        for x in 0..(max_x as usize) {
            if claim_counts[x][y] >= 2 {
                claim_multiples += 1;
            }
        }
    }

    claim_multiples
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let claims: Vec<Claim> = input.lines().map(|x| parse(x)).collect();

    println!(
        "Part 1: {} square inches of fabric are within two or more claims",
        solve_part1(&claims)
    );
}
