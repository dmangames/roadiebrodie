use std::env;
use std::path::Path;
use std::process::Command;


fn main() {
    // Ensure we are in the project directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_dir = Path::new(&manifest_dir);

    #[cfg(target_os = "windows")]
    let npx_name = "npx.cmd";

    #[cfg(not(target_os = "windows"))]
    let npx_name = "npx";

    //Run postcss to compile Tailwind CSS
    let tailwind_output = Command::new(npx_name)
        .arg("tailwindcss")
        .arg("--config")
        .arg("tailwind/config.js")
        .arg("--input")
        .arg("tailwind/styles.css")
        .arg("--output")
        .arg("static/styles.css")
        .current_dir(&project_dir)
        .output()
        .expect("Failed to compile tailwind");


    if !tailwind_output.status.success() {
        panic!(
            "Tailwind compilation failed:\n{}",
            String::from_utf8_lossy(&tailwind_output.stderr)
        );
    }

    // Inform cargo about the output directory
    println!("cargo:rerun-if-changed=tailwind/styles.css");
    println!("cargo:rerun-if-changed=tailwind/config.js");
}


