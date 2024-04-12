use crate::parser::Operation;

use std::io::{self, Read};

fn is_overflow(pointer: &usize, max_pointer: &usize) {
    if *pointer >= *max_pointer {
        eprintln!("\n    WTF: Fucking overflow !!");
        std::process::exit(1);
    }
}

pub fn brainfuck(operations: &Vec<Operation>, tape: &mut Vec<u8>, pointer: &mut usize) {
    for operation in operations {
        match operation {
            Operation::IncrementPointer => {
                *pointer += 1;
                is_overflow(&pointer, &tape.len());
            },
            Operation::DecrementPointer => {
                *pointer -= 1;
                is_overflow(&pointer, &tape.len());
            }
            Operation::IncrementData => tape[*pointer] += 1,
            Operation::DecrementData => tape[*pointer] -= 1,
            Operation::Print => print!("{}", tape[*pointer] as char),
            Operation::Input => {
                let mut input = [0u8];
                match io::stdin().read_exact(&mut input) {
                    Ok(_) => {
                        tape[*pointer] = input[0];
                    },
                    Err(_) => {
                        eprintln!("\n    WTF: Fucking input !!\n");
                        std::process::exit(1);
                    },
                }
            },
            Operation::Loop(operations) => {
                while tape[*pointer] != 0 {
                    brainfuck(operations, tape, pointer);
                }
            }
        }
    }
}

