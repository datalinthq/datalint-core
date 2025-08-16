fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=Rstrtmgr");
        // Also ensure other Windows libraries are linked
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=crypt32");
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        // Set explicit platform to bypass CXX ABI check
        println!("cargo:rustc-env=DUCKDB_EXPLICIT_PLATFORM=1");
        // Use specific compiler flags for ARM64
        println!("cargo:rustc-env=CXXFLAGS=-DDUCKDB_EXPLICIT_PLATFORM");
    }
}
