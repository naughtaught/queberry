fn main() {
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::fs;
        use std::path::PathBuf;

        println!("cargo:rerun-if-changed=mpv");
        let lib_path = PathBuf::from("mpv");
        if lib_path.exists() {
            println!("cargo:rustc-link-search=native={}", lib_path.display());
            println!("cargo:rustc-link-lib=mpv");
        } else {
            eprintln!("Warning: mpv directory not found");
        }

        let dll_src = lib_path.join("libmpv-2.dll");
        if dll_src.exists() {
            let out_dir = env::var("OUT_DIR").unwrap();
            let target_dir = PathBuf::from(&out_dir)
                .ancestors()
                .nth(3)
                .unwrap()
                .to_path_buf();
            let dll_dest = target_dir.join("libmpv-2.dll");
            match fs::copy(&dll_src, &dll_dest) {
                Ok(_) => println!("Copied libmpv-2.dll to {:?}", dll_dest),
                Err(e) => eprintln!("Warning: Failed to copy DLL: {}", e),
            }
        } else {
            eprintln!("Warning: libmpv-2.dll not found at {}", dll_src.display());
        }
    }
    tauri_build::build()
}
