use pest::Parser;
use pest_derive::Parser;

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
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Text(String),
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
                                        for argument in function_call_part.into_inner() {
                                            match argument.as_rule() {
                                                Rule::string => {
                                                    function_call.arguments.push(Expression::Text(
                                                        argument
                                                            .into_inner()
                                                            .next()
                                                            .unwrap()
                                                            .as_str()
                                                            .to_owned(),
                                                    ));
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            ast.nodes.push(Statement::FunctionCall(function_call));
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
