use self::optimizations::optimize_arguments::optimze_arguments;
use crate::frontend::ast::{Ast, Statement};

mod optimizations;

pub fn middleend(ast: &mut Ast) {
    for node in &mut ast.nodes {
        match node {
            Statement::FunctionCall(function_call) => {
                optimze_arguments(&mut function_call.arguments)
            }
            Statement::ActionCall(action_call) => optimze_arguments(&mut action_call.arguments),
        }
    }
}
