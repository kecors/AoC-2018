use pom::parser::*;
use pom::Error;
use std::io::{stdin, Read};
use std::str;

type Value = u8;
type Opcode = u8;
type Register = usize;
type RegisterOrValue = usize;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Contents {
    registers: Vec<Value>,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    a: RegisterOrValue,
    b: RegisterOrValue,
    c: Register,
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

#[derive(Debug)]
struct Engine {
    samples: Vec<Sample>,
    program: Program,
}

impl Engine {
    fn sample_behaviour_count(&self) -> u32 {
        let mut sample_count = 0;

        for sample in self.samples.iter() {
            let ops = [
                addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir,
                eqri, eqrr,
            ];
            let mut op_count = 0;

            for op in ops.iter() {
                if op(&sample.before, &sample.instruction) == sample.after {
                    op_count += 1;
                }
            }

            if op_count >= 3 {
                sample_count += 1;
            }
        }

        sample_count
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, u8> {
    let number = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..2)) | sym(b'0');
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| u8::from_str_radix(s, 10))
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
        opcode: numbers[0],
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
    (samples() + program()).map(|(samples, program)| Engine { samples, program })
}

fn addr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = contents.registers[instruction.b];
    result.registers[instruction.c] = a + b;

    result
}

fn addi(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = instruction.b as u8;
    result.registers[instruction.c] = a + b;

    result
}

fn mulr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = contents.registers[instruction.b];
    result.registers[instruction.c] = a * b;

    result
}

fn muli(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = instruction.b as u8;
    result.registers[instruction.c] = a * b;

    result
}

fn banr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = contents.registers[instruction.b];
    result.registers[instruction.c] = a & b;

    result
}

fn bani(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = instruction.b as u8;
    result.registers[instruction.c] = a & b;

    result
}

fn borr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = contents.registers[instruction.b];
    result.registers[instruction.c] = a | b;

    result
}

fn bori(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    let b = instruction.b as u8;
    result.registers[instruction.c] = a | b;

    result
}

fn setr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    let a = contents.registers[instruction.a];
    result.registers[instruction.c] = a;

    result
}

fn seti(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    result.registers[instruction.c] = instruction.a as u8;

    result
}

fn gtir(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if instruction.a as u8 > contents.registers[instruction.b] {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn gtri(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if contents.registers[instruction.a] > instruction.b as u8 {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn gtrr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if contents.registers[instruction.a] > contents.registers[instruction.b] {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn eqir(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if instruction.a as u8 == contents.registers[instruction.b] {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn eqri(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if contents.registers[instruction.a] == instruction.b as u8 {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn eqrr(contents: &Contents, instruction: &Instruction) -> Contents {
    let mut result: Contents = (*contents).clone();

    if contents.registers[instruction.a] == contents.registers[instruction.b] {
        result.registers[instruction.c] = 1;
    } else {
        result.registers[instruction.c] = 0;
    }

    result
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = engine().parse(input.as_bytes())?;

    let count = engine.sample_behaviour_count();
    println!(
        "Part 1: {} samples behave like three or more opcodes",
        count
    );

    Ok(())
}
