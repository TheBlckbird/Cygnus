use crate::frontend::ast::{Arguments, Expression};

use super::optimize_string::optimize_string;

pub fn optimze_arguments(arguments: &mut Arguments) {
    for argument in arguments {
        match argument {
            Expression::String(string_node) => optimize_string(string_node),
        }
    }
}
