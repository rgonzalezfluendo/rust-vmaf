// REASON: It is not harmful to panic in the compile time with _reasonable_ error.
#![allow(clippy::expect_used)]
use std::{env, path::PathBuf, process::Command};

use bindgen::Builder;

const VMAF_HEADERS: [&str; 5] = [
    "feature.h",
    "libvmaf_cuda.h",
    "libvmaf.h",
    "model.h",
    "picture.h",
];

const VENV_NAME: &str = ".venv";

fn git_fetch(repo: &str, branch: &str, output: &str) {
    #[rustfmt::skip]
    Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--recurse-submodules",
            repo,
            output,
        ])
        .status()
        .expect("Unable to clone repository.");
    Command::new("git")
        .args(["checkout", branch])
        .status()
        .expect("Unable to checkout repository.");
}

fn output() -> PathBuf {
    PathBuf::from(
        env::var("OUT_DIR").expect("Cannot fail because cargo sets this for the build script."),
    )
}

fn main() {
    let mut vmaf_path = output();
    vmaf_path.push("vmaf");

    git_fetch(
        "https://github.com/Netflix/vmaf.git",
        "v3.0.0",
        &vmaf_path.to_string_lossy(),
    );

    let mut vmaf_include = vmaf_path.clone();
    ["libvmaf", "include"]
        .into_iter()
        .for_each(|item| vmaf_include.push(item));
    let bindings = Builder::default().clang_args(["-I", &vmaf_include.to_string_lossy()]);
    vmaf_include.push("libvmaf");

    let mut bindings = bindings.clang_args(["-I", &vmaf_include.to_string_lossy()]);
    for header in VMAF_HEADERS {
        let mut tmp = vmaf_include.clone();
        tmp.push(header);
        bindings = bindings.header(tmp.to_string_lossy());
    }

    bindings
        .allowlist_function("vmaf_.*")
        .allowlist_type("Vmaf.*")
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file(format!("{}/vmaf.rs", output().to_string_lossy()))
        .expect("Unable to write generated bindings.");

    assert!(Command::new("python3")
        .args(["-mvenv", VENV_NAME])
        .current_dir(&vmaf_path)
        .status()
        .expect("Unable to execute command.")
        .success());
    // Upstream VMAF does not install `setuptools` package
    // via pip and expects it to be already installed.
    assert!(Command::new(format!("{VENV_NAME}/bin/pip"))
        .args(["install", "setuptools"])
        .current_dir(&vmaf_path)
        .status()
        .expect("Unable to execute command.")
        .success());
    assert!(Command::new("make")
        .args([&format!("VENV={VENV_NAME}"), "-j"])
        .current_dir(&vmaf_path)
        .status()
        .expect("Unable to execute command.")
        .success());

    println!(
        "cargo:rustc-link-search=native={}/libvmaf/build/src/",
        vmaf_path.to_string_lossy()
    );
    if std::env::var("CARGO_FEATURE_STATIC_LINK").is_ok() {
        println!("cargo:rustc-link-lib=static=vmaf");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=stdc++");
    } else {
        println!("cargo:rustc-link-lib=vmaf");
    }
}
