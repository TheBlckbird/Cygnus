use pest::iterators::Pair;

use super::{
    ast::{ArrayType, DictionaryType, Identifier, Type},
    Rule,
};

pub fn parse_dictionary_type(dictionary_pair: Pair<Rule>) -> DictionaryType {
    let entries = dictionary_pair.into_inner();
    let mut dictionary = DictionaryType::new();

    for entry in entries {
        let mut identifier = Identifier::new();
        let mut type_ = Type::String;

        for entry_part in entry.into_inner() {
            match entry_part.as_rule() {
                Rule::identifier => {
                    identifier = entry_part.as_str().to_owned();
                }
                Rule::r#type => {
                    // match entry_part.as_str() {
                    //     "string" => {
                    //         type_ = Type::String;
                    //     }
                    //     "number" => {
                    //         type_ = Type::Number;
                    //     }
                    //     "date" => {
                    //         type_ = Type::Date;
                    //     }
                    //     "data" => {
                    //         type_ = Type::Data;
                    //     }
                    //     "boolean" => {
                    //         type_ = Type::Boolean;
                    //     }
                    //     // "Array" => {
                    //     //     type_ = Type::Array(
                    //     //         Vec::new(),
                    //     //     );
                    //     // }
                    //     _ if entry_part.as_str().starts_with("dictionary") => {
                    //         type_ = Type::Dictionary(parse_dictionary_type(
                    //             entry_part.into_inner().next().unwrap(),
                    //         ));
                    //     }
                    //     _ => unreachable!(),
                    // }
                    type_ = parse_type(entry_part);
                }
                _ => unreachable!(),
            }
        }

        dictionary.push_entry(identifier, type_);
    }

    dictionary
}

pub fn parse_array_type(array: Pair<Rule>) -> ArrayType {
    let entries = array.into_inner();
    let mut array = ArrayType::new();

    for entry in entries {
        array.push_entry(parse_type(entry));
    }

    array
}

pub fn parse_type(type_: Pair<Rule>) -> Type {
    return match type_.as_str() {
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
    };
}
