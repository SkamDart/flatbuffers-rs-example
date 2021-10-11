use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    Command::new("flatc")
        .arg("-r")
        .arg("-o")
        .arg(std::env::var_os("OUT_DIR").unwrap())
        .arg("apis/hello.fbs")
        .status()
        .expect("Failed to call flatc");
}
