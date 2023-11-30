use compiler::{compiler, Workflow};
use parser::parser;
use plist::to_file_binary;
use std::fs;

mod compiler;
mod error;
// mod new_action;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("dev_assets/main.scs").unwrap();
    let ast = parser(file_content.as_str());
    let workflow = compiler(ast);

    to_file_binary::<_, Workflow>("dev_assets/compiled.shortcut", &workflow)?;

    Ok(())
}
