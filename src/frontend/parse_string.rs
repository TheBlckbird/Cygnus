use pest::{iterators::Pair, Span};

use crate::frontend::Position;

use super::{Rule, StringNode};

pub fn parse_string(string: Pair<Rule>) -> StringNode {
    // dbg!(&string);

    let mut string_node = StringNode::new(Position::from(string.line_col()));

    let string_parts = match string.as_rule() {
        Rule::string => string.into_inner(),
        _ => unreachable!(),
    };

    for string_part in string_parts {
        match string_part.as_rule() {
            Rule::literal_string => {}
            Rule::expression => {}
            _ => unreachable!(),
        }
    }

    string_node
}
