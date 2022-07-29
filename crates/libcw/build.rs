use cbindgen::{DocumentationStyle, Language};
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = PathBuf::from(".")
        .join(format!("{}.h", package_name))
        .display()
        .to_string();
    let config = cbindgen::Config {
        documentation_style: DocumentationStyle::Doxy,
        language: Language::C,
        pragma_once: true,
        line_length: 80,
        ..Default::default()
    };
    cbindgen::generate_with_config(crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}
