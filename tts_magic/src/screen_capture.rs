use anyhow::Result;
use fs_extra::dir;
use std::time::Instant;
use xcap::Monitor;
use log::{info, debug, error};

use std::process::Command;

pub fn capture_screen() -> Result<()> {
    let output = Command::new("gnome-screenshot")
        .args(&["-f", "output/screenshot.png"])
        .output()
        .expect("Failed to run gnome-screenshot");

    if output.status.success() {
        println!("Screenshot saved as screenshot.png");
    } else {
        eprintln!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
