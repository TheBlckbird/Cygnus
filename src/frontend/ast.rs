use std::fmt::Display;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Statement>,
    pub macros: Vec<MacroDefinition>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodes: vec![],
            macros: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
    MacroCall(MacroCall),
}

#[derive(Debug)]
pub enum Argument {
    Expression(Expression),
    String(String),
}

impl Argument {
    pub fn as_type(&self) -> Type {
        match self {
            Argument::String(_) => Type::String,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub struct MacroDefinition {
    pub macro_identifier: Identifier,
    pub action_id: String,
    pub action_arguments: DictionaryType,
}

#[derive(Debug)]
pub struct MacroCall {
    pub macro_name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum Expression {}

pub type Identifier = String;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String,
    Number,
    Date,
    Data,
    Boolean,
    Array(ArrayType),
    Dictionary(DictionaryType),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "string"),
            Type::Number => write!(f, "number"),
            Type::Date => write!(f, "date"),
            Type::Data => write!(f, "data"),
            Type::Boolean => write!(f, "boolean"),
            Type::Array(array_type) => {
                write!(f, "[")?;

                for (index, entry) in array_type.entries.iter().enumerate() {
                    write!(f, "{}", entry)?;

                    if index != array_type.entries.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
            Type::Dictionary(dictionary_type) => {
                write!(f, "{{")?;

                for (index, (identifier, type_)) in dictionary_type.entries.iter().enumerate() {
                    write!(f, "{}: {}", identifier, type_)?;

                    if index != dictionary_type.entries.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayType {
    entries: Vec<Type>,
}

impl ArrayType {
    pub fn new() -> ArrayType {
        ArrayType { entries: vec![] }
    }

    pub fn push_entry(&mut self, type_: Type) {
        self.entries.push(type_);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DictionaryType {
    pub entries: Vec<(Identifier, Type)>,
    index: usize,
}

impl DictionaryType {
    pub fn new() -> DictionaryType {
        DictionaryType {
            entries: vec![],
            index: 0,
        }
    }

    pub fn push_entry(&mut self, identifier: Identifier, type_: Type) {
        self.entries.push((identifier, type_));
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Iterator for DictionaryType {
    type Item = (Identifier, Type);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len() {
            let result = self.entries[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display_works() {
        let type_ = Type::Array(ArrayType::new());
        assert_eq!(type_.to_string(), "[]");

        let mut array_type = ArrayType::new();
        array_type.push_entry(Type::String);
        array_type.push_entry(Type::Number);
        array_type.push_entry(Type::Date);
        array_type.push_entry(Type::Data);
        array_type.push_entry(Type::Boolean);
        array_type.push_entry(Type::Array(ArrayType::new()));
        let mut dictionary_type = DictionaryType::new();
        dictionary_type.push_entry("string".to_owned(), Type::String);
        array_type.push_entry(Type::Dictionary(dictionary_type));

        let type_ = Type::Array(array_type);
        assert_eq!(
            type_.to_string(),
            "[string, number, date, data, boolean, [], {string: string}]"
        );
    }
}
