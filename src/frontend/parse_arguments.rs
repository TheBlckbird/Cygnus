use super::{
    ast::Position, parse_string::parse_string, Expression, NonStringExpression, StringNode,
};
use crate::frontend::Rule;
use pest::iterators::Pair;

pub fn parse_arguments(call_part: Pair<Rule>) -> Vec<Expression> {
    let mut arguments = vec![]; //::new_string(Position::from(call_part.line_col()));

    for argument in call_part.into_inner() {
        let expression = match argument.as_rule() {
            Rule::expression => argument.into_inner().next().unwrap(),
            _ => unreachable!(),
        };

        match expression.as_rule() {
            Rule::string => {
                let string = parse_string(expression);

                // arguments.push(Expression::String(StringNode::new(Position::from(
                //     expression.as_span().start_pos().line_col(),
                // ))));
            }
            _ => {
                unreachable!();
            }
        }

        // let position = Position::from(argument.line_col());

        // arguments.push(Expression::String(
        //     argument.into_inner().next().unwrap().as_str().to_owned(),
        //     position,
        // ));
    }

    arguments
}
