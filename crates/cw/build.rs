use clap::crate_name;
use clap::load_yaml;
use clap::App;
use clap::Shell;
use std::env;

fn main() {
    let dir = env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(||env::var_os("OUT_DIR"))
        .or_else(||Some("target/sh".parse().unwrap()));

    let output_dir = match dir {
        None => return,
        Some(x) => x,
    };
    let _ignore = std::fs::create_dir_all(&output_dir);
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let mut app = App::from_yaml(yaml);

    app.gen_completions(crate_name!(), Shell::Zsh, &output_dir);
    app.gen_completions(crate_name!(), Shell::Bash, &output_dir);
    app.gen_completions(crate_name!(), Shell::PowerShell, &output_dir);
    app.gen_completions(crate_name!(), Shell::Fish, &output_dir);
}
