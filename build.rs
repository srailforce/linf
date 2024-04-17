use std::{env, path::Path};


fn main() {
    #[cfg(windows)]
    if env::var("TARGET").expect("target").ends_with("windows-msvc") {
            let manifest = Path::new("manifest.xml").canonicalize().unwrap();
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest.display());
            println!("cargo:rerun-if-changed=manifest.xml");
        }
}