use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead, Read};

enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpZero(usize),
    JumpNotZero(usize)
}

fn parse(input_file: File) -> Result<Vec<Instruction>, String> {
    let mut pos: usize = 0;
    let mut stack: Vec<usize> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in BufReader::new(input_file).lines().map(|l| l.unwrap()) {
        for c in line.chars() {
            match c {
                '>' => instructions.push(Instruction::MoveRight),
                '<' => instructions.push(Instruction::MoveLeft),
                '+' => instructions.push(Instruction::Increment),
                '-' => instructions.push(Instruction::Decrement),
                '.' => instructions.push(Instruction::Write),
                ',' => instructions.push(Instruction::Read),
                '[' => {
                    stack.push(pos);
                    instructions.push(Instruction::JumpZero(0));
                },
                ']' => {
                    if let Some(p) = stack.pop() {
                        instructions[p] = Instruction::JumpZero(pos);
                        instructions.push(Instruction::JumpNotZero(p))
                    } else {
                        return Err("Unmatched jump instructions.".to_owned());
                    }
                },
                 _ => pos -= 1
            }

            pos += 1;
        }
    }

    if stack.len() > 0 {
        return Err("Unmatched jump instructions.".to_owned());
    }

    Ok(instructions)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Expected an input file to run.".to_owned());
    }

    let input_file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to open input file: {}", err.description()))
    };

    let instructions = parse(input_file)?;
    let mut tape = vec![0u8; 25];

    let mut pc: usize = 0;
    let mut pointer: usize = 0;
    let program_size = instructions.len();

    while pc < program_size {
        match instructions[pc] {
            Instruction::MoveRight => pointer += 1,
            Instruction::MoveLeft => pointer -= 1,
            Instruction::Increment => tape[pointer] = tape[pointer].wrapping_add(1u8),
            Instruction::Decrement => tape[pointer] = tape[pointer].wrapping_sub(1u8),
            Instruction::Write => print!("{}", tape[pointer] as char),
            Instruction::Read => {
                let mut buffer = [0u8];

                tape[pointer] = if let Ok(_) = std::io::stdin().read_exact(&mut buffer) {
                    buffer[0]
                } else {
                    0
                };
            },
            Instruction::JumpZero(pos) => {
                if tape[pointer] == 0 {
                    pc = pos;
                }
            },
            Instruction::JumpNotZero(pos) => {
                if tape[pointer] != 0 {
                    pc = pos;
                }
            }
        }

        pc += 1;
    }

    Ok(())
}

