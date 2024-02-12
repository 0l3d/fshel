mod shell;

use shell::fshell::*;

fn main() {
    init_shell(format!("$ `{}` : ", current_path()));
}