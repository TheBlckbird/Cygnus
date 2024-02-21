use crate::frontend::ast::{Expression, StringNode, Type};

pub fn optimize_string(string: &mut StringNode) {
    let string_parts = &mut string.0;

    let mut previous_string_part_was_string = false;

    for (index, string_part) in string_parts.iter_mut().enumerate() {}
}
