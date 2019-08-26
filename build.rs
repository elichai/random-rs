use std::process::Command;
use std::{env, str};

fn main() {
    if let Some(v) = rustc_version() {
        if v >= 26 {
            println!("cargo:rustc-cfg=feature=\"u128\"");
        }
    }
}

fn rustc_version() -> Option<u32> {
    if let Some(rustc) = env::var_os("RUSTC") {
        if let Some(output) = Command::new(rustc).arg("--version").output().ok() {
            if let Some(version) = str::from_utf8(&output.stdout).ok() {
                let mut pieces = version.split('.');
                if pieces.next() != Some("rustc 1") {
                    return None;
                }
                if let Some(piece) = pieces.next() {
                    return piece.parse().ok();
                }
            }
        }
    }
    return None;
}
