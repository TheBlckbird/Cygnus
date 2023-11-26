#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Token {
    Symbol { content: String },
    StringLiteral { content: String },
    OpeningParenthesis { content: String },
    ClosingParenthesis { content: String },
    EOF,
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut i = 0;
    let mut tokens: Vec<Token> = vec![];

    while i < input.len() - 1 {
        let current_char = input.chars().nth(i).unwrap();
        let allowed_identifier_characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

        let mut token: Option<Token> = None;

        match current_char {
            '"' => {
                i += 1;
                let string_literal = parse_string_literal(input, &mut i);
                token = Some(Token::StringLiteral {
                    content: string_literal,
                });
            }
            _ if allowed_identifier_characters.contains(current_char) => {
                let symbol = parse_symbol(input, &mut i, allowed_identifier_characters);
                token = Some(Token::Symbol { content: symbol });
                i -= 1;
            }
            '(' => {
                token = Some(Token::OpeningParenthesis {
                    content: "(".to_owned(),
                });
            }
            ')' => {
                token = Some(Token::ClosingParenthesis {
                    content: ")".to_owned(),
                });
            }
            '\n' => {}
            _ => {
                panic!(
                    "Unexpected character \"{}\" at position {}",
                    current_char, i
                )
            }
        }

        if let Some(token) = token {
            tokens.push(token);
        }

        i += 1;
    }

    tokens.push(Token::EOF);

    tokens
}

fn parse_string_literal(input: &str, index: &mut usize) -> String {
    let mut output = String::new();

    loop {
        let current_char = input.chars().nth(*index).unwrap();
        if current_char == '"' {
            break;
        }

        output.push(current_char);
        *index += 1;
    }

    output
}

fn parse_symbol(input: &str, index: &mut usize, allowed_identifier_characters: &str) -> String {
    let mut output = String::new();

    loop {
        let current_char = input.chars().nth(*index).unwrap();
        if !allowed_identifier_characters.contains(current_char) {
            break;
        }

        output.push(current_char);
        *index += 1;
    }

    output
}
