use std::io::Write;
use std::process::{Command, Stdio};

fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let output = if is_wayland() {
        Command::new("wl-paste").output()?
    } else {
        Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()?
    };

    if !output.status.success() {
        return Err("Failed to read from clipboard.".into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = if is_wayland() {
        Command::new("wl-copy").arg(text).status()?
    } else {
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?
    };

    if !status.success() {
        return Err("Failed to write to clipboard.".into());
    }
    Ok(())
}