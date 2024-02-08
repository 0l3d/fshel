use std::path::Path;
use std::env;

use rsbash::rash;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let (pwdcode, pwd, pwderr) = rash!("pwd").expect("ERROR RASH USING PWD"); 
        if pwdcode != 0 {
            print!("{}", pwdcode);
        }
        if pwderr != "" {
            println!("{}", pwderr);
        }
        let shell_text = format!("$ {} ~ % ", pwd);
        let readline = rl.readline(shell_text.as_str());
        let mut val = 0;
        let mut err : String = String::new();
        let mut out : String = String::new();
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
                    _ => (val, out, err) = rash!(line).expect("ERROR"),
                };
                if out != "" {
                    print!("{}", out);
                }
                if err != "" {
                    println!("{}", err);
                }
                if val != 0 {
                    println!("Exit Code : {}", val);
                }
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
