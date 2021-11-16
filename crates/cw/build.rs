use clap::crate_name;
use clap::load_yaml;
use clap::App;
use clap::Shell;
use std::env;

fn main() {
    let dir = env::var_os("SHELL_COMPLETIONS_DIR")
        .or(env::var_os("OUT_DIR"))
        .or(Some("target/sh".parse().unwrap()));

    let outdir = match dir {
        None => return,
        Some(outdir) => outdir,
    };
    let _ignore = std::fs::create_dir_all(&outdir);
    let yaml = load_yaml!("resources/cmdline-clap.yaml");
    let mut app = App::from_yaml(yaml);

    app.gen_completions(crate_name!(), Shell::Zsh, &outdir);
    app.gen_completions(crate_name!(), Shell::Bash, &outdir);
    app.gen_completions(crate_name!(), Shell::PowerShell, &outdir);
    app.gen_completions(crate_name!(), Shell::Fish, &outdir);
}
