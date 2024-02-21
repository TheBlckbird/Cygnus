use super::{
    ast::{ArrayType, DictionaryType, Identifier, Position, Type},
    Rule,
};
use pest::iterators::Pair;

pub fn parse_dictionary_type(dictionary_pair: Pair<Rule>) -> DictionaryType {
    let mut dictionary = DictionaryType::new(Position::from(dictionary_pair.line_col()));
    let entries = dictionary_pair.into_inner();

    for entry in entries {
        let mut identifier = Identifier::new();
        let mut type_ = Type::String;

        let position = Position::from(entry.line_col());

        for entry_part in entry.into_inner() {
            match entry_part.as_rule() {
                Rule::identifier => {
                    identifier = entry_part.as_str().to_owned();
                }
                Rule::r#type => {
                    type_ = parse_type(entry_part);
                }
                _ => unreachable!(),
            }
        }

        dictionary.push_entry(identifier, type_, position);
    }

    dictionary
}

pub fn parse_array_type(array: Pair<Rule>) -> ArrayType {
    let mut array_type = ArrayType::new(Position::from(array.line_col()));
    let entries = array.into_inner();

    for entry in entries {
        array_type.push_entry(parse_type(entry));
    }

    array_type
}

pub fn parse_type(type_: Pair<Rule>) -> Type {
    match type_.as_str() {
        "string" => Type::String,
        "number" => Type::Number,
        "date" => Type::Date,
        "data" => Type::Data,
        "boolean" => Type::Boolean,
        _ if type_.as_str().starts_with("array") => {
            Type::Array(parse_array_type(type_.into_inner().next().unwrap()))
        }
        _ if type_.as_str().starts_with("dictionary") => {
            Type::Dictionary(parse_dictionary_type(type_.into_inner().next().unwrap()))
        }
        _ => unreachable!(),
    }
}
