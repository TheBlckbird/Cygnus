use std::{env::args, fs};

use crate::lexer::Lexer;
use compiler::{compiler, Workflow};
use parser::parser;
use plist::to_file_binary;

mod compiler;
mod error;
mod lexer;
// mod new_action;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut plist: Workflow = from_file("dev_assets/modified.shortcut")
    // .expect("The file you provided is not a valid shortcut");

    let file_content = fs::read_to_string("dev_assets/main.scs").unwrap();
    let mut lexer = Lexer::new(&file_content);
    let ast = parser(&mut lexer);
    let workflow = compiler(ast);

    to_file_binary::<_, Workflow>("dev_assets/compiled.shortcut", &workflow)?;

    Ok(())
}
