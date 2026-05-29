use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let docs_dir = manifest_dir.join("./assets/docs");

    println!("cargo::rerun-if-changed=./assets/docs/src");
    println!("cargo::rerun-if-changed=./assets/docs/book.toml");

    require_mdbook();

    let status = Command::new("mdbook")
        .arg("build")
        .arg(&docs_dir)
        .status()
        .expect("failed to invoke `mdbook`");

    if !status.success() {
        panic!("`mdbook build` failed with status: {status}");
    }
}

fn require_mdbook() {
    let installed = Command::new("mdbook")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    if !installed {
        panic!("`mdbook` not found on PATH. Install it with `cargo install mdbook --locked`.");
    }
}
