use std::char;
use std::io::{stdin, Read};
use std::u32;

#[derive(Debug)]
struct Engine {
    board: Vec<u32>,
    elf_a: usize,
    elf_b: usize,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            board: vec![3, 7],
            elf_a: 0,
            elf_b: 1,
        }
    }

    fn generate_new_recipes(&mut self) -> Vec<u32> {
        let mut new_recipes = Vec::new();

        let sum = self.board[self.elf_a] + self.board[self.elf_b];
        if sum >= 10 {
            new_recipes.push(1);
        }
        new_recipes.push(sum % 10);

        new_recipes
    }

    fn set_current_recipes(&mut self) {
        self.elf_a = (self.elf_a + 1 + self.board[self.elf_a] as usize) % self.board.len();
        self.elf_b = (self.elf_b + 1 + self.board[self.elf_b] as usize) % self.board.len();
    }

    fn solve_part1(&mut self, recipe_count: usize) -> String {
        for _ in 0..(recipe_count + 10) {
            let mut new_recipes = self.generate_new_recipes();
            self.board.append(&mut new_recipes);
            self.set_current_recipes();
            //self.display();
        }

        self.board[recipe_count..(recipe_count + 10)]
            .iter()
            .map(|&x| char::from_digit(x, 10).unwrap())
            .collect()
    }

    fn solve_part2(&mut self, score_sequence: &[u32]) -> usize {
        loop {
            for recipe in self.generate_new_recipes() {
                //self.display();
                self.board.push(recipe);
                if self.board.len() < score_sequence.len() {
                    continue;
                }
                let left_recipe_count = self.board.len() - score_sequence.len();
                let mut discrepancy_flag = false;
                for j in 0..score_sequence.len() {
                    if score_sequence[j] != self.board[left_recipe_count + j] {
                        discrepancy_flag = true;
                        break;
                    }
                }
                if !discrepancy_flag {
                    return left_recipe_count;
                }
            }
            self.set_current_recipes();
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for (j, recipe) in self.board.iter().enumerate() {
            if self.elf_a == j {
                print!("(");
            }
            if self.elf_b == j {
                print!("[");
            }
            print!("{}", recipe);
            if self.elf_b == j {
                print!("]");
            }
            if self.elf_a == j {
                print!(")");
            }
            print!(" ");
        }
        println!("\n");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new();
    let recipe_count: usize = input.trim().parse().unwrap();
    let part1 = engine.solve_part1(recipe_count);
    println!(
        "Part 1: after the first {} recipes, the scores of the next ten recipes are '{}'",
        recipe_count, part1
    );

    let mut engine = Engine::new();
    let score_sequence: Vec<u32> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let part2 = engine.solve_part2(&score_sequence);
    println!(
        "Part 2: {} recipes appear on the scoreboard to the left of '{}'",
        part2,
        input.trim()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_9() {
        let mut engine = Engine::new();
        assert_eq!(engine.solve_part1(9), String::from("5158916779"));
    }

    #[test]
    fn test_part1_5() {
        let mut engine = Engine::new();
        assert_eq!(engine.solve_part1(5), String::from("0124515891"));
    }

    #[test]
    fn test_part1_18() {
        let mut engine = Engine::new();
        assert_eq!(engine.solve_part1(18), String::from("9251071085"));
    }

    #[test]
    fn test_part1_2018() {
        let mut engine = Engine::new();
        assert_eq!(engine.solve_part1(2018), String::from("5941429882"));
    }

    #[test]
    fn test_part2_51589() {
        let mut engine = Engine::new();
        let score_sequence = vec![5, 1, 5, 8, 9];
        assert_eq!(engine.solve_part2(&score_sequence), 9);
    }

    #[test]
    fn test_part2_01245() {
        let mut engine = Engine::new();
        let score_sequence = vec![0, 1, 2, 4, 5];
        assert_eq!(engine.solve_part2(&score_sequence), 5);
    }

    #[test]
    fn test_part2_92510() {
        let mut engine = Engine::new();
        let score_sequence = vec![9, 2, 5, 1, 0];
        assert_eq!(engine.solve_part2(&score_sequence), 18);
    }

    #[test]
    fn test_part2_59414() {
        let mut engine = Engine::new();
        let score_sequence = vec![5, 9, 4, 1, 4];
        assert_eq!(engine.solve_part2(&score_sequence), 2018);
    }
}
