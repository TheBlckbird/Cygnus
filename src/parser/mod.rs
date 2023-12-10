mod parse_arguments;

use pest::Parser;
use pest_derive::Parser;

use crate::parser::parse_arguments::parse_arguments;

#[derive(Parser)]
#[grammar = "short_script.pest"]
struct ShortScriptParser;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { nodes: vec![] }
    }
}

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
    MacroCall(MacroCall),
    MacroDefinition(MacroDefinition),
}

#[derive(Debug)]
pub enum Argument {
    Expression(Expression),
    Text(Text),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub struct MacroDefinition {
    pub macro_name: String,
    pub identifier: String,
    pub parameters: Vec<String>,
}

#[derive(Debug)]
pub struct MacroCall {
    pub macro_name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum Expression {}

pub type Identifier = String;
pub type Text = String;

pub enum Type {
    String,
    Number,
    Date,
    Data,
    Boolean,
    Array(Vec<Type>),
    Dictionary(Vec<(Identifier, Type)>),
}

pub fn parser(input: &str) -> Ast {
    let program = ShortScriptParser::parse(Rule::program, input)
        .unwrap_or_else(|e| panic!("{}", e))
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
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    println!("{:#?}", ast);
    ast
}
