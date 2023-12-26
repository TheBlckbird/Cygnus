pub mod ast;
mod parse_arguments;
mod parse_statements;
mod parse_types;

use crate::parser::ast::*;
use pest::Parser;
use pest_derive::Parser;

use self::{
    ast::Ast,
    parse_statements::{parse_function_call, parse_macro_call, parse_macro_definition},
};

#[derive(Parser)]
#[grammar = "short_script.pest"]
struct ShortScriptParser;

pub fn parser(input: &str) -> Ast {
    let program = ShortScriptParser::parse(Rule::program, input)
        .unwrap_or_else(|e| panic!("{e}"))
        .next()
        .unwrap();

    let mut ast = Ast::new();

    for statement in program.into_inner() {
        match statement.as_rule() {
            Rule::statement => {
                for statement_part in statement.into_inner() {
                    match statement_part.as_rule() {
                        Rule::function_call => {
                            ast.nodes
                                .push(Statement::FunctionCall(parse_function_call(statement_part)));
                        }
                        Rule::macro_call => {
                            ast.nodes
                                .push(Statement::MacroCall(parse_macro_call(statement_part)));
                        }
                        Rule::macro_definition => {
                            ast.macros
                                .push(parse_macro_definition(statement_part, &ast.macros));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    ast
}
