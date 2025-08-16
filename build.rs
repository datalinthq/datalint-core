use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // Print cargo instructions for rerun
    println!("cargo:rerun-if-changed=build.rs");

    match target_os.as_str() {
        "windows" => {
            // Windows-specific libraries required by DuckDB
            println!("cargo:rustc-link-lib=Rstrtmgr");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=crypt32");

            // Additional Windows libraries that might be needed
            println!("cargo:rustc-link-lib=bcrypt");
            println!("cargo:rustc-link-lib=ntdll");
            println!("cargo:rustc-link-lib=userenv");
        }
        "macos" => {
            // macOS specific settings
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Security");

            // Handle both x86_64 and aarch64 (Apple Silicon)
            if target_arch == "aarch64" {
                // Apple Silicon specific flags if needed
                println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
            } else {
                // Intel Mac
                println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
            }
        }
        "linux" => {
            // Linux typically doesn't need special linking for DuckDB when bundled
            // But we can add common libraries just in case
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=m");
        }
        _ => {
            // Other platforms - basic settings
            println!(
                "cargo:warning=Building for untested platform: {}",
                target_os
            );
        }
    }

    // Set optimization flags for DuckDB bundled build
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=DUCKDB_COMPILE_FLAGS=-O3");
    }
}
