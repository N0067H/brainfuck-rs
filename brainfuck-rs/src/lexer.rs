
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementData,
    DecrementData,
    Print,
    Input,
    LoopOpen,
    LoopClose,
}

pub fn lexer(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for characters in code.chars() {
        let token = match characters {
            '>' => Some(Token::IncrementPointer),
            '<' => Some(Token::DecrementPointer),
            '+' => Some(Token::IncrementData),
            '-' => Some(Token::DecrementData),
            '.' => Some(Token::Print),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopOpen),
            ']' => Some(Token::LoopClose),
            _ => None,
        };

        match token {
            Some(token) => tokens.push(token),
            None => (),
        }
    }

    tokens
}

