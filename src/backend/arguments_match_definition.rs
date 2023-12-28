use crate::frontend::ast::{Argument, AsType, DictionaryType};

pub fn arguments_match_definition(
    call_arguments: &Vec<Argument>,
    definition_parameters: &DictionaryType,
) -> bool {
    if call_arguments.len() != definition_parameters.len() {
        return false;
    }

    for (index, argument) in call_arguments.iter().enumerate() {
        if argument.as_type() != definition_parameters.entries[index].1 {
            return false;
        }
    }

    true
}
