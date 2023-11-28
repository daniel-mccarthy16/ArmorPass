mod encryption;
mod generator;
pub mod password_manager;
pub mod shell;
mod utility;

use crate::shell::Shell;

fn main() {
    let mut armor_pass_shell = Shell::new();
    armor_pass_shell.run();
}
