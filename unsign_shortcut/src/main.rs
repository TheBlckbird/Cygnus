use reqwest::blocking::get;
use serde::Deserialize;
use std::{env::args, io::stdin, path::Path, process::exit};

#[derive(Debug, Deserialize)]
struct SignedShortcut {
    fields: Fields,
}

#[derive(Debug, Deserialize)]
struct Fields {
    shortcut: Shortcut,
}

#[derive(Debug, Deserialize)]
struct Shortcut {
    value: Value,
}

#[derive(Debug, Deserialize)]
struct Value {
    #[serde(rename = "downloadURL")]
    download_url: String,
}

fn main() {
    let arguments_len = args().len() - 1;

    if arguments_len != 2 {
        eprintln!("Expected 1 argument, got {arguments_len}");
        println!("Format: [iCloud link] [output path]");
        exit(1);
    }

    let new_link = format!(
        "https://www.icloud.com/shortcuts/api/records/{}",
        args().nth(1).unwrap().split('/').last().unwrap()
    );

    let download_url = match get(new_link) {
        Ok(return_json) => match return_json.json::<SignedShortcut>() {
            Ok(value) => value.fields.shortcut.value.download_url,
            Err(err) => {
                eprintln!("{err}");
                exit(1);
            }
        },
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };

    let file_content = match get(download_url) {
        Ok(response) => response.bytes().unwrap(),
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };

    let output_path = args().nth(2).unwrap();
    if Path::new(&output_path).exists() {
        print!("There already exists a file or folder called {output_path}. Do you want to overwrite it? [y/N]: ");

        let mut buffer = String::new();

        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer).unwrap();
        let overwrite = match buffer.trim_end() {
            "y" | "Y" => true,
            "n" | "N" | "" => false,
            _ => {
                println!("Invalid input. Exiting...");
                exit(1);
            }
        };

        if !overwrite {
            println!("Exiting...");
            exit(0);
        }
    }

    match std::fs::write(&output_path, file_content) {
        Ok(_) => {
            println!("Written to {}", output_path);
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
}
