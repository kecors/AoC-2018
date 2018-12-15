use std::char;
use std::io::{stdin, Read};

fn solve(recipe_number: u32) -> String {
    let mut board: Vec<u32> = vec![3, 7];
    let mut elf_a: usize = 0;
    let mut elf_b: usize = 1;

    for _ in 0..(recipe_number + 10) {
        let sum = board[elf_a] + board[elf_b];
        if sum >= 10 {
            board.push(1);
        }
        board.push(sum % 10);

        elf_a = (elf_a + 1 + board[elf_a] as usize) % board.len();
        elf_b = (elf_b + 1 + board[elf_b] as usize) % board.len();

        if false {
            // Display board
            for (j, recipe) in board.iter().enumerate() {
                if elf_a == j {
                    print!("(");
                }
                if elf_b == j {
                    print!("[");
                }
                print!("{}", recipe);
                if elf_b == j {
                    print!("]");
                }
                if elf_a == j {
                    print!(")");
                }
                print!(" ");
            }
            println!();
        }
    }

    let range = recipe_number as usize..(recipe_number as usize + 10);
    board[range]
        .iter()
        .map(|&x| char::from_digit(x, 10).unwrap())
        .collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let recipe_number: u32 = input.trim().parse().unwrap();
    let part1 = solve(recipe_number);
    println!(
        "Part 1: the scores of the ten recipes immediately after are '{}'",
        part1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(solve(9), String::from("5158916779"));
    }

    #[test]
    fn test_5() {
        assert_eq!(solve(5), String::from("0124515891"));
    }

    #[test]
    fn test_18() {
        assert_eq!(solve(18), String::from("9251071085"));
    }

    #[test]
    fn test_2018() {
        assert_eq!(solve(2018), String::from("5941429882"));
    }
}
