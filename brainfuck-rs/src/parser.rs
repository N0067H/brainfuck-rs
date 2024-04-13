
use crate::lexer::Token;

#[derive(Debug)]
#[derive(Clone)]
pub enum Operation {
    IncrementPointer,
    DecrementPointer,
    IncrementData,
    DecrementData,
    Print,
    Input,
    Loop(Vec<Operation>)
}

pub fn parser(tokens: &[Token]) -> Vec<Operation> {
    let mut loop_begin: usize = 0;
    let mut loop_depth: usize = 0;

    let mut operations: Vec<Operation> = Vec::new();
    for (position, token) in tokens.iter().enumerate() {
        if loop_depth == 0 {
            let operation = match token {
                Token::IncrementPointer => Some(Operation::IncrementPointer),
                Token::DecrementPointer => Some(Operation::DecrementPointer),
                Token::IncrementData => Some(Operation::IncrementData),
                Token::DecrementData => Some(Operation::DecrementData),
                Token::Print => Some(Operation::Print),
                Token::Input => Some(Operation::Input),
                Token::LoopBegin => {
                    loop_begin = position;
                    loop_depth += 1;
                    None
                },
                Token::LoopEnd => {
                    eprintln!("\n    WTF: Fucking LoopOpen !!\n");
                    std::process::exit(1);
                },
            };

            match operation {
                Some(operation) => operations.push(operation),
                None => (),
            }
        } else {
            match token {
                Token::LoopBegin => loop_depth += 1,
                Token::LoopEnd => {
                    loop_depth -= 1;
                    
                    if loop_depth == 0 {
                        let loop_end = position;
                        let loop_code = parser(&tokens[loop_begin + 1..loop_end]);
                        operations.push(Operation::Loop(loop_code));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_depth != 0 {
        eprintln!("\n    WTF: Fucking LoopClose !!\n");
        std::process::exit(1);
    }

    operations
}

