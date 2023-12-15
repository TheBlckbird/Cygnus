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
    MacroDefinition(MacroDefinition),
}

#[derive(Debug)]
pub enum Argument {
    Expression(Expression),
    String(String),
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
    pub action_parameters: DictionaryType,
}

#[derive(Debug)]
pub struct MacroCall {
    pub macro_name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum Expression {}

pub type Identifier = String;

#[derive(Debug)]
pub enum Type {
    String,
    Number,
    Date,
    Data,
    Boolean,
    Array(ArrayType),
    Dictionary(DictionaryType),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DictionaryType {
    entries: Vec<(Identifier, Type)>,
}

impl DictionaryType {
    pub fn new() -> DictionaryType {
        DictionaryType { entries: vec![] }
    }

    pub fn push_entry(&mut self, identifier: Identifier, type_: Type) {
        self.entries.push((identifier, type_));
    }
}
