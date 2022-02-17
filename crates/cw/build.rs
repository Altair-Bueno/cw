use std::env;

use clap::{Command, CommandFactory};
use clap_complete::{generate_to, Shell};

include!("src/config.rs");

const BIN_NAME:&str = env!("CARGO_PKG_NAME");

const SHELL_LIST:&[Shell] = &[
    Shell::Bash,
    Shell::Elvish,
    Shell::Fish,
    Shell::PowerShell,
    Shell::Zsh,
];

fn main() {
    let mut command :Command = Config::command();
    let target_dir = env::var("OUT_DIR").unwrap();
    for shell in SHELL_LIST {
        let out_dir = format!("{target_dir}/completions/{shell}");
        let _ = std::fs::create_dir_all(&out_dir);
        generate_to(shell.clone(), &mut command, BIN_NAME, out_dir).unwrap();
    }
}
