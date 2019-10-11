extern crate cbindgen;

use std::env;

fn main() {
    let release_profile = env::var("PROFILE")
        .map(|x| match x.to_lowercase().as_ref() {
            "release" => true,
            _ => false
        })
        .unwrap_or(false);
    if release_profile {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR");
        let generated = cbindgen::Builder::new()
            .with_crate(&crate_dir)
            .with_documentation(true)
            .generate()
            .expect("c/c++ header file generation failed");
        let mut filepath = target_dir();
        filepath.push("include");
        std::fs::create_dir_all(&filepath).expect("unable to create include dir");
        filepath.push("web_images_cabi.h");
        generated.write_to_file(filepath);
    }
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> std::path::PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        let mut out = std::path::PathBuf::from(target);
        out.push("release");
        out
    } else {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR");
        let mut out = std::path::PathBuf::from(crate_dir);
        out.push("target");
        out.push("release");
        out
    }
}
