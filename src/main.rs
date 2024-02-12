mod shell;

use shell::fshell::*;

fn main() {
    let _ = init_shell(format!("$ `{}` : ", current_path()));
}