// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[no_mangle]
pub unsafe extern "C" fn res_init() -> std::ffi::c_int {
    extern "C" {
        fn __res_init() -> std::ffi::c_int;
    }
    __res_init()
}

fn main() {
    sieve::run();
}

