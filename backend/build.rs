fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("linux") && target.contains("gnu") {
        println!("cargo:rustc-link-lib=resolv");
    }

    #[cfg(feature = "gui")]
    tauri_build::build();
}
