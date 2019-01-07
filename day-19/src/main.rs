use pom::parser::*;
use pom::Error;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug)]
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
struct Instruction {
    operator: Operator,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
struct Engine {
    ip_register: usize,
    ip: usize,
    registers: Vec<usize>,
    instructions: Vec<Instruction>,
}

impl Engine {
    fn new(ip_register: usize, instructions: Vec<Instruction>) -> Engine {
        let ip = 0;
        let registers = vec![0; 6];

        Engine {
            ip_register,
            ip,
            registers,
            instructions,
        }
    }

    fn run(&mut self) {
        loop {
            if self.ip > self.instructions.len() {
                return;
            }

            self.registers[self.ip_register] = self.ip;

            operate(&self.instructions[self.ip], &mut self.registers);

            self.ip = self.registers[self.ip_register];
            self.ip += 1;
        }
    }
}

fn operate(instruction: &Instruction, registers: &mut [usize]) {
    use self::Operator::*;

    match instruction.operator {
        Addr => {
            let a = registers[instruction.a];
            let b = registers[instruction.b];
            registers[instruction.c] = a + b;
        }
        Addi => {
            let a = registers[instruction.a];
            let b = instruction.b;
            registers[instruction.c] = a + b;
        }
        Mulr => {
            let a = registers[instruction.a];
            let b = registers[instruction.b];
            registers[instruction.c] = a * b;
        }
        Muli => {
            let a = registers[instruction.a];
            let b = instruction.b;
            registers[instruction.c] = a * b;
        }
        Banr => {
            let a = registers[instruction.a];
            let b = registers[instruction.b];
            registers[instruction.c] = a & b;
        }
        Bani => {
            let a = registers[instruction.a];
            let b = instruction.b;
            registers[instruction.c] = a & b;
        }
        Borr => {
            let a = registers[instruction.a];
            let b = registers[instruction.b];
            registers[instruction.c] = a | b;
        }
        Bori => {
            let a = registers[instruction.a];
            let b = instruction.b;
            registers[instruction.c] = a | b;
        }
        Setr => {
            let a = registers[instruction.a];
            registers[instruction.c] = a;
        }
        Seti => {
            registers[instruction.c] = instruction.a;
        }
        Gtir => {
            if instruction.a > registers[instruction.b] {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
        Gtri => {
            if registers[instruction.a] > instruction.b {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
        Gtrr => {
            if registers[instruction.a] > registers[instruction.b] {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
        Eqir => {
            if instruction.a == registers[instruction.b] {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
        Eqri => {
            if registers[instruction.a] == instruction.b {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
        Eqrr => {
            if registers[instruction.a] == registers[instruction.b] {
                registers[instruction.c] = 1;
            } else {
                registers[instruction.c] = 0;
            }
        }
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, usize> {
    let number = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| usize::from_str_radix(s, 10))
}

fn ip_register<'a>() -> Parser<'a, u8, usize> {
    seq(b"#ip ") * number().map(|x| x as usize)
}

fn operator<'a>() -> Parser<'a, u8, Operator> {
    seq(b"addr").map(|_| Operator::Addr)
        | seq(b"addi").map(|_| Operator::Addi)
        | seq(b"mulr").map(|_| Operator::Mulr)
        | seq(b"muli").map(|_| Operator::Muli)
        | seq(b"banr").map(|_| Operator::Banr)
        | seq(b"bani").map(|_| Operator::Bani)
        | seq(b"borr").map(|_| Operator::Borr)
        | seq(b"bori").map(|_| Operator::Bori)
        | seq(b"setr").map(|_| Operator::Setr)
        | seq(b"seti").map(|_| Operator::Seti)
        | seq(b"gtir").map(|_| Operator::Gtir)
        | seq(b"gtri").map(|_| Operator::Gtri)
        | seq(b"gtrr").map(|_| Operator::Gtrr)
        | seq(b"eqir").map(|_| Operator::Eqir)
        | seq(b"eqri").map(|_| Operator::Eqri)
        | seq(b"eqrr").map(|_| Operator::Eqrr)
}

fn instruction<'a>() -> Parser<'a, u8, Instruction> {
    (operator() + ((space() * number()) + ((space() * number()) + (space() * number()))))
        .map(|(operator, (a, (b, c)))| Instruction { operator, a, b, c })
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (ip_register() + (space() * instruction()).repeat(1..))
        .map(|(ip_register, instructions)| Engine::new(ip_register, instructions))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;

    engine.run();
    println!(
        "Part 1: the value left in register 0 is {}",
        engine.registers[0]
    );

    Ok(())
}
