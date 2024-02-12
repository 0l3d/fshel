mod shell;

use std::path::Path;
use std::env;

use std::{thread, time};

pub use shell::bash::*;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let shell_text = format!("$ ~ % ");
        thread::sleep(time::Duration::from_millis(2));
        let readline = rl.readline(shell_text.as_str());
        // let mut commandout: String = String::new();
        match readline {
            Ok(line) => {
                let command_split: Vec<&str> = line.split(" ").collect();
                match command_split[0] {
                    "cd" => {
                        if command_split.len() == 2 {
                            let path = Path::new(command_split[1]);
                            if path.is_dir() {
                                env::set_current_dir(&path).expect("ERROR CD");
                            }
                        }
                    },
                    "exit" => break,
                    _ => run(line.to_string()),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
