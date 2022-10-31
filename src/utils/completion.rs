use std::io;

use clap::Command;
use clap_complete::{generate, shells, Generator};
use strum::EnumString;

#[derive(EnumString, Debug, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum ShellType {
    Bash,
    Fish,
    Zsh,
    PowerShell,
    Elvish,
}

impl ShellType {
    pub fn generate_completion(&self, name: &str, cmd: &mut Command) {
        match self {
            ShellType::Bash => gen(shells::Bash, name, cmd),
            ShellType::Fish => gen(shells::Fish, name, cmd),
            ShellType::Zsh => gen(shells::Zsh, name, cmd),
            ShellType::PowerShell => gen(shells::PowerShell, name, cmd),
            ShellType::Elvish => gen(shells::Elvish, name, cmd),
        }
    }
}

fn gen<G: Generator>(shell: G, name: &str, cmd: &mut Command) {
    generate(shell, cmd, name, &mut io::stdout());
}
