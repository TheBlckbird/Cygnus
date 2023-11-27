#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Symbol { content: String },
    StringLiteral { content: String },
    OpeningParenthesis { content: String },
    ClosingParenthesis { content: String },
    EOF,
}

pub struct Lexer<'a> {
    current_char_index: usize,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            current_char_index: 0,
            input,
        }
    }

    pub fn next(&mut self) -> Token {
        loop {
            let current_char_option = self.input.chars().nth(self.current_char_index);
            if current_char_option.is_none() {
                return Token::EOF;
            }

            let current_char = current_char_option.unwrap();

            let allowed_identifier_characters =
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

            let mut token: Option<Token> = None;

            match current_char {
                '"' => {
                    self.current_char_index += 1;
                    let string_literal =
                        Self::parse_string_literal(self.input, &mut self.current_char_index);
                    token = Some(Token::StringLiteral {
                        content: string_literal,
                    });
                }
                _ if allowed_identifier_characters.contains(current_char) => {
                    let symbol = Self::parse_symbol(
                        self.input,
                        &mut self.current_char_index,
                        allowed_identifier_characters,
                    );
                    token = Some(Token::Symbol { content: symbol });
                    self.current_char_index -= 1;
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
                '\n' | ' ' | ' ' => {}
                _ => {
                    panic!(
                        "Unexpected character \"{}\" at position {}",
                        current_char, self.current_char_index
                    )
                }
            }

            self.current_char_index += 1;

            if let Some(token) = token {
                return token;
            }
        }
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
}
