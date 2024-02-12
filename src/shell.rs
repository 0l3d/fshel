pub mod bash {
    use std::process::Command;

    pub fn run(shell_command : String) {
        let shell_command_with_arguments: Vec<&str> = shell_command.split(" ").collect();
        let mut arguments : Vec<String> = vec![String::new(); 0];
        for i in 1..shell_command_with_arguments.len() {
            arguments.push(shell_command_with_arguments[i].to_string());
        }
        Command::new(shell_command_with_arguments[0]).args(arguments).spawn().expect("Failed to execute process : ");
   }
   pub fn current_path() -> String {
        return "".to_string();
   }
}