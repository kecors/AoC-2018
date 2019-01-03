use pom::parser::*;
use pom::Error;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Contents {
    registers: Vec<u32>,
}

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
struct Sample {
    before: Contents,
    instruction: Instruction,
    after: Contents,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Operator {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug)]
struct Engine {
    samples: Vec<Sample>,
    sample_valid_operators: HashMap<usize, HashSet<Operator>>,
    opcodes: Vec<Option<Operator>>,
    program: Program,
}

impl Engine {
    fn generate_sample_valid_operators(&mut self) {
        use self::Operator::*;

        let operators = [
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
            Eqri, Eqrr,
        ];

        for (index, sample) in self.samples.iter().enumerate() {
            for operator in operators.iter() {
                if operate(*operator, &sample.before, &sample.instruction) == sample.after {
                    let svo = self
                        .sample_valid_operators
                        .entry(index)
                        .or_insert_with(HashSet::new);
                    svo.insert(*operator);
                }
            }
        }
    }

    fn generate_opcodes(&mut self) {
        let mut work_remains_flag = true;

        while work_remains_flag {
            work_remains_flag = false;

            for (sample_index, operators) in self.sample_valid_operators.iter() {
                match operators.len() {
                    0 => {}
                    1 => {
                        let opcode = self.samples[*sample_index].instruction.opcode;
                        let operator = *operators.iter().next().unwrap();
                        match self.opcodes[opcode] {
                            None => {
                                self.opcodes[opcode] = Some(operator);
                            }
                            Some(operator_) => {
                                if operator_ != operator {
                                    panic!("One opcode associated with multiple operators");
                                }
                            }
                        }
                    }
                    _ => {
                        work_remains_flag = true;
                    }
                }
            }

            for (_, operators) in self.sample_valid_operators.iter_mut() {
                self.opcodes.iter().for_each(|x| {
                    if let Some(opcode) = x {
                        operators.remove(opcode);
                    }
                });
            }
        }
    }

    fn run_program(&self) -> Contents {
        let mut contents = Contents {
            registers: vec![0; 4],
        };

        for instruction in self.program.instructions.iter() {
            if let Some(opcode) = self.opcodes[instruction.opcode] {
                contents = operate(opcode, &contents, &instruction);
            }
        }

        contents
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, u32> {
    let number = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..2)) | sym(b'0');
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| u32::from_str_radix(s, 10))
}

fn contents<'a>() -> Parser<'a, u8, Contents> {
    (sym(b'[') * number() + (sym(b',') * space() * number()).repeat(3) - sym(b']')).map(
        |(first_number, other_numbers)| {
            let mut registers = vec![first_number];
            registers.extend(other_numbers);
            Contents { registers }
        },
    )
}

fn instruction<'a>() -> Parser<'a, u8, Instruction> {
    ((space() * number()).repeat(4)).map(|numbers| Instruction {
        opcode: numbers[0] as usize,
        a: numbers[1] as usize,
        b: numbers[2] as usize,
        c: numbers[3] as usize,
    })
}

fn sample<'a>() -> Parser<'a, u8, Sample> {
    let before = space() * seq(b"Before:") * space() * contents();
    let instruction = space() * instruction();
    let after = space() * seq(b"After:") * space() * contents();

    (before + instruction + after).map(|((before, instruction), after)| Sample {
        before,
        instruction,
        after,
    })
}

fn samples<'a>() -> Parser<'a, u8, Vec<Sample>> {
    sample().repeat(1..).map(|samples| samples)
}

fn program<'a>() -> Parser<'a, u8, Program> {
    instruction()
        .repeat(1..)
        .map(|instructions| Program { instructions })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (samples() + program()).map(|(samples, program)| Engine {
        samples,
        sample_valid_operators: HashMap::new(),
        opcodes: vec![None; 16],
        program,
    })
}

fn operate(operator: Operator, contents: &Contents, instruction: &Instruction) -> Contents {
    use self::Operator::*;

    let mut result: Contents = (*contents).clone();

    match operator {
        Addr => {
            let a = contents.registers[instruction.a];
            let b = contents.registers[instruction.b];
            result.registers[instruction.c] = a + b;
        }
        Addi => {
            let a = contents.registers[instruction.a];
            let b = instruction.b as u32;
            result.registers[instruction.c] = a + b;
        }
        Mulr => {
            let a = contents.registers[instruction.a];
            let b = contents.registers[instruction.b];
            result.registers[instruction.c] = a * b;
        }
        Muli => {
            let a = contents.registers[instruction.a];
            let b = instruction.b as u32;
            result.registers[instruction.c] = a * b;
        }
        Banr => {
            let a = contents.registers[instruction.a];
            let b = contents.registers[instruction.b];
            result.registers[instruction.c] = a & b;
        }
        Bani => {
            let a = contents.registers[instruction.a];
            let b = instruction.b as u32;
            result.registers[instruction.c] = a & b;
        }
        Borr => {
            let a = contents.registers[instruction.a];
            let b = contents.registers[instruction.b];
            result.registers[instruction.c] = a | b;
        }
        Bori => {
            let a = contents.registers[instruction.a];
            let b = instruction.b as u32;
            result.registers[instruction.c] = a | b;
        }
        Setr => {
            let a = contents.registers[instruction.a];
            result.registers[instruction.c] = a;
        }
        Seti => {
            result.registers[instruction.c] = instruction.a as u32;
        }
        Gtir => {
            if instruction.a as u32 > contents.registers[instruction.b] {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
        Gtri => {
            if contents.registers[instruction.a] > instruction.b as u32 {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
        Gtrr => {
            if contents.registers[instruction.a] > contents.registers[instruction.b] {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
        Eqir => {
            if instruction.a as u32 == contents.registers[instruction.b] {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
        Eqri => {
            if contents.registers[instruction.a] == instruction.b as u32 {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
        Eqrr => {
            if contents.registers[instruction.a] == contents.registers[instruction.b] {
                result.registers[instruction.c] = 1;
            } else {
                result.registers[instruction.c] = 0;
            }
        }
    }

    result
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;

    engine.generate_sample_valid_operators();

    // For part 1, count samples with three or more valid operators
    let mut count = 0;
    for (_, operators) in engine.sample_valid_operators.iter() {
        if operators.len() >= 3 {
            count += 1;
        }
    }
    println!(
        "Part 1: {} samples behave like three or more opcodes",
        count
    );

    engine.generate_opcodes();
    let contents = engine.run_program();
    println!(
        "Part 2: the value contained in register 0 after running the test program is {}",
        contents.registers[0]
    );

    Ok(())
}
