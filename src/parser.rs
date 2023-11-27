use crate::{
    error::error,
    lexer::{Lexer, Token},
};

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Expression>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { nodes: vec![] }
    }
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Text(String),
}

pub fn parser(lexer: &mut Lexer) -> Ast {
    let mut ast = Ast::new();

    loop {
        let token = lexer.next();
        if token == Token::EOF {
            break;
        }

        #[allow(clippy::single_match)]
        match token {
            Token::Symbol {
                content: symbol_name,
            } => {
                if let Token::OpeningParenthesis { content: _ } = lexer.next() {
                    if let Token::StringLiteral {
                        content: string_literal,
                    } = lexer.next()
                    {
                        if let Token::ClosingParenthesis { content: _ } = lexer.next() {
                            ast.nodes.push(Expression::FunctionCall(FunctionCall {
                                function_name: symbol_name,
                                arguments: vec![Statement::Text(string_literal)],
                            }));
                        } else {
                            error("Expected closing parenthesis \")\"".to_owned());
                        }
                    } else {
                        error("Expected statement after \"(\"".to_owned());
                    }
                } else {
                    error("Expected opening parenthesis \"(\"".to_owned());
                }
            }
            _ => {}
        }
    }

    ast
}
