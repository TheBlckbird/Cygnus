use std::fmt::Display;
use std::slice::{Iter, IterMut};

pub trait AsType {
    fn as_type(&self) -> Type;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub fn new(line: usize, character: usize) -> Self {
        Position { line, character }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.line, self.character)
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            line: value.0,
            character: value.1,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Statement>,
    pub actions: Vec<ActionDefinition>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodes: vec![],
            actions: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
    ActionCall(ActionCall),
}

#[derive(Debug)]
pub struct Arguments {
    pub position: Position,
    pub arguments: Vec<Argument>,
    index: usize,
}

impl Arguments {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            arguments: Vec::new(),
            index: 0,
        }
    }

    pub fn new_blank() -> Self {
        Self {
            position: Position::new(0, 0),
            arguments: Vec::new(),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.arguments.len()
    }

    pub fn iter(&self) -> Iter<'_, Argument> {
        self.arguments.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Argument> {
        self.arguments.iter_mut()
    }
}

#[derive(Debug)]
pub enum Argument {
    Expression(Expression, Position),
    String(String, Position),
}

impl Argument {
    pub fn position(&self) -> &Position {
        match self {
            Argument::Expression(_, pos) => pos,
            Argument::String(_, pos) => pos,
        }
    }
}

impl AsType for Argument {
    fn as_type(&self) -> Type {
        match self {
            Argument::String(_, _) => Type::String,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Arguments,
    pub position: Position,
}

#[derive(Debug)]
pub struct ActionDefinition {
    pub action_identifier: Identifier,
    pub action_id: String,
    pub action_arguments: DictionaryType,
    pub position: Position,
}

#[derive(Debug)]
pub struct ActionCall {
    pub action_name: String,
    pub arguments: Arguments,
    pub position: Position,
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

                for (index, (identifier, type_, _)) in dictionary_type.entries.iter().enumerate() {
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
    pub position: Position,
}

impl ArrayType {
    pub fn new(position: Position) -> ArrayType {
        ArrayType {
            entries: vec![],
            position,
        }
    }

    pub fn push_entry(&mut self, type_: Type) {
        self.entries.push(type_);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DictionaryType {
    pub entries: Vec<(Identifier, Type, Position)>,
    index: usize,
    pub position: Position,
}

impl DictionaryType {
    pub fn new(position: Position) -> DictionaryType {
        DictionaryType {
            entries: vec![],
            index: 0,
            position,
        }
    }

    pub fn push_entry(&mut self, identifier: Identifier, type_: Type, position: Position) {
        self.entries.push((identifier, type_, position));
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Iterator for DictionaryType {
    type Item = (Identifier, Type, Position);

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
        let type_ = Type::Array(ArrayType::new(Position {
            line: 0,
            character: 0,
        }));
        assert_eq!(type_.to_string(), "[]");

        let mut array_type = ArrayType::new(Position {
            line: 0,
            character: 0,
        });
        array_type.push_entry(Type::String);
        array_type.push_entry(Type::Number);
        array_type.push_entry(Type::Date);
        array_type.push_entry(Type::Data);
        array_type.push_entry(Type::Boolean);
        array_type.push_entry(Type::Array(ArrayType::new(Position {
            line: 0,
            character: 0,
        })));
        let mut dictionary_type = DictionaryType::new(Position {
            line: 0,
            character: 0,
        });
        dictionary_type.push_entry(
            "string".to_owned(),
            Type::String,
            Position {
                line: 0,
                character: 0,
            },
        );
        array_type.push_entry(Type::Dictionary(dictionary_type));

        let type_ = Type::Array(array_type);
        assert_eq!(
            type_.to_string(),
            "[string, number, date, data, boolean, [], {string: string}]"
        );
    }
}
