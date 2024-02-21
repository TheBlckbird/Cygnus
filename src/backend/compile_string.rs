use plist::Dictionary;

use super::StringNode;

#[allow(non_snake_case)]
pub struct Text {
    WFSerializationType: String,
    Value: Value,
}

impl Text {
    pub fn new(value: Value) -> Self {
        Text {
            WFSerializationType: "WFTextTokenString".to_owned(),
            Value: value,
        }
    }
}

#[allow(non_snake_case)]
pub struct Value {
    string: String,
    AttachmentsByRange: Dictionary,
}

#[allow(non_snake_case)]
pub struct StringPart {
    OuputUUID: String,
    Type: String,
    OutputName: String,
}

pub fn compile_string(string: StringNode) /*-> Text*/ {}
