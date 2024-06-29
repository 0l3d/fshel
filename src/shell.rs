pub mod fshell {
    use std::process::Command;

    use std::env;
    use std::path::Path;

    use std::{thread, time};

    use rustyline::error::ReadlineError;
    use rustyline::{DefaultEditor, Result};

    pub fn init_shell(init_text : String) -> Result<()> {
        let mut rl = DefaultEditor::new()?;
        loop {
            let shell_text = init_text.clone();
            thread::sleep(time::Duration::from_millis(4));
            let readline = rl.readline(shell_text.as_str());

            match readline {
                Ok(line) => {
                    let command_split: Vec<&str> = line.split(" ").collect();
                    match command_split[0] {
                        // CUSTOM COMMANDS
                        "cd" => {
                            if command_split.len() == 2 {
                                let path = Path::new(command_split[1]);
                                if path.is_dir() {
                                    env::set_current_dir(&path).expect("ERROR CD");
                                }
                            }
                        },
                        "exit" => break,
                        _ => {
                            let shell_command_with_arguments: Vec<&str> = line.split(" ").collect();
                            let mut arguments: Vec<String> = vec![String::new(); 0];
                            for i in 1..shell_command_with_arguments.len() {
                                arguments.push(shell_command_with_arguments[i].to_string());
                            }
                            let _child = match Command::new(shell_command_with_arguments[0]).args(arguments).spawn() {
                                Ok(child) =>  {
                                    child.wait_with_output()
                                },
                                Err(e) => {
                                    println!("ERROR: {}", e);
                                    continue;
                                },
                            };  
                        },
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
}
