use clap::{arg, command, value_parser, Arg, ArgAction};
use compiler::{compiler, Workflow};
use parser::parser;
use plist::to_file_binary;
use std::{
    fs,
    path::PathBuf,
    process::{exit, Command},
};
use tempfile::tempdir;

mod compiler;
mod error;
// mod new_action;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(
            arg!([input] "The file you want to compile")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <OUTPUT> "The location of where the output file should go"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        // .arg(arg!(-n --no_sign "Disables automatic signing of the shortcut (Needed if your using another OS than MacOS)").action(ArgAction::SetTrue))
        .arg(
            Arg::new("no-sign")
                .short('n')
                .long("no-sign")
                .help("Disables automatic signing of the shortcut (Needed if your using another OS than macOS)")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("Compiling...");
    let input_file = matches.get_one::<PathBuf>("input").unwrap();
    let output_location = matches.get_one::<PathBuf>("output").unwrap();

    let file_content = fs::read_to_string(input_file).unwrap();
    let ast = parser(file_content.as_str());
    let workflow = compiler(ast);

    let sign_shortcut = !matches.get_flag("no-sign");
    if sign_shortcut && cfg!(target_os = "macos") {
        println!("Signing...");

        let temp_dir = tempdir().expect("Couldn’t create temp dir");
        let unsigned_location = temp_dir.path().join("unsigned.shortcut");

        to_file_binary::<_, Workflow>(&unsigned_location, &workflow)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "shortcuts sign -i {} -o {}",
                unsigned_location.display(),
                output_location.display()
            ))
            .output()
            .expect("Couldn’t sign shortcut");

        if !output.stderr.is_empty() {
            println!("Something went wrong while trying to sign the shortcut:");
            print!("{}", String::from_utf8(output.stderr).unwrap());
            exit(1);
        }
    } else if sign_shortcut {
        println!("You can only sign shortcuts on macOS")
    } else {
        to_file_binary::<_, Workflow>(output_location, &workflow)?;
    }

    println!("Done");

    Ok(())
}
