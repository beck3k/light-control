use std::env;

fn main() {
    if cfg!(target_os = "macos") {
        // Check for virtual environment
        if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}/lib", venv_path);
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}/Library/Frameworks", venv_path);
        }

        // Fallback paths
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/Library/Frameworks");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
    }
} 