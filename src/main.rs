mod shell;

use shell::fshell::*;

use std::env;

fn main() {
    let _ = init_shell(format!("$ <{}> % ", (env::current_dir().expect("Failed get current directory for this program shell.")).display()));
}