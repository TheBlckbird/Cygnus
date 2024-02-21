use self::{
    ast::Ast,
    parse_statements::{parse_action_call, parse_action_definition, parse_function_call},
    type_checker::type_checker,
};
use crate::frontend::ast::*;
use pest::Parser;
use pest_derive::Parser;
use std::error::Error;

pub mod ast;
mod parse_arguments;
mod parse_statements;
mod parse_string;
mod parse_types;
mod type_checker;

#[derive(Parser)]
#[grammar = "short_script.pest"]
struct ShortScriptParser;

pub fn parser(input: &str) -> Result<Ast, Vec<Box<impl Error>>> {
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
                        Rule::action_call => {
                            ast.nodes
                                .push(Statement::ActionCall(parse_action_call(statement_part)));
                        }
                        Rule::action_definition => {
                            ast.actions
                                .push(parse_action_definition(statement_part, &ast.actions));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    let type_checker_result = type_checker(&ast);

    if let Err(type_checker_errors) = type_checker_result {
        let mut errors = vec![];

        for type_checker_error in type_checker_errors {
            errors.push(type_checker_error);
        }

        return Err(errors);
    }

    // dbg!(&ast);

    Ok(ast)
}
