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
    dbg!(program);

    ast
}
