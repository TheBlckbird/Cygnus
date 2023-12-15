pub mod ast;
mod parse_arguments;
mod parse_types;

use std::process::exit;

use crate::{
    error::error,
    parser::{ast::*, parse_arguments::parse_arguments},
};
use pest::Parser;
use pest_derive::Parser;

use self::{ast::Ast, parse_types::parse_dictionary_type};

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
                            let mut function_call = FunctionCall {
                                function_name: String::new(),
                                arguments: Vec::new(),
                            };

                            for function_call_part in statement_part.into_inner() {
                                match function_call_part.as_rule() {
                                    Rule::identifier => {
                                        function_call.function_name =
                                            function_call_part.as_str().to_owned();
                                    }
                                    Rule::arguments => {
                                        function_call.arguments =
                                            parse_arguments(function_call_part);
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            ast.nodes.push(Statement::FunctionCall(function_call));
                        }
                        Rule::macro_call => {
                            let mut macro_call = MacroCall {
                                macro_name: String::new(),
                                arguments: Vec::new(),
                            };

                            for macro_call_part in statement_part.into_inner() {
                                match macro_call_part.as_rule() {
                                    Rule::identifier => {
                                        macro_call.macro_name = macro_call_part.as_str().to_owned();
                                    }
                                    Rule::arguments => {
                                        macro_call.arguments = parse_arguments(macro_call_part);
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            ast.nodes.push(Statement::MacroCall(macro_call));
                        }
                        Rule::macro_definition => {
                            let mut macro_definition = MacroDefinition {
                                macro_identifier: Identifier::new(),
                                action_id: String::new(),
                                action_parameters: DictionaryType::new(),
                            };

                            for macro_definition_part in statement_part.into_inner() {
                                match macro_definition_part.as_rule() {
                                    Rule::identifier => {
                                        if ast.macros.iter().any(|m| {
                                            m.macro_identifier == macro_definition_part.as_str()
                                        }) {
                                            // TODO:TODO: Implement proper error handling!
                                            dbg!(&ast.macros);
                                            error(format!(
                                                "Macro \"{}\" is defined multiple times!",
                                                macro_definition_part.as_str()
                                            ));
                                        }

                                        macro_definition.macro_identifier =
                                            macro_definition_part.as_str().to_owned();
                                    }
                                    Rule::macro_body => {
                                        for macro_body_part in macro_definition_part.into_inner() {
                                            match macro_body_part.as_rule() {
                                                Rule::macro_action_id => {
                                                    macro_definition.action_id = macro_body_part
                                                        .into_inner()
                                                        .next()
                                                        .unwrap()
                                                        .into_inner()
                                                        .next()
                                                        .unwrap()
                                                        .as_str()
                                                        .to_owned();
                                                }
                                                Rule::macro_action_parameters => {
                                                    let entries = macro_body_part
                                                        .into_inner()
                                                        .next()
                                                        .unwrap();

                                                    macro_definition.action_parameters =
                                                        parse_dictionary_type(entries);
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            ast.macros.push(macro_definition);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    dbg!(&ast);
    ast
}
