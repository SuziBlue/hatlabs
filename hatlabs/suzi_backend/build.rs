use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Only rerun if frontend changes
    println!("cargo:rerun-if-changed=../suzi_web/");

    // Step 1: Build the frontend with Trunk
    let status = Command::new("trunk")
        .arg("build")
        .arg("--release")
        .current_dir("../suzi_web")
        .status()
        .expect("Failed to run trunk build");

    if !status.success() {
        panic!("Trunk build failed");
    }

    // Step 2: Copy frontend/dist/* to server/cdn/* (without deleting existing files)
    let dist_dir = Path::new("../suzi_web/dist");
    let cdn_dir = Path::new("cdn");

    // Create cdn directory if missing
    if !cdn_dir.exists() {
        fs::create_dir_all(&cdn_dir).expect("Failed to create cdn directory");
    }

    copy_dir_recursive(dist_dir, cdn_dir).expect("Failed to copy frontend files to cdn");
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
