use std::io::Write;
use std::process::{Command, Stdio};

enum DisplayServer {
    Wayland,
    X11,
}

fn get_display_server() -> Result<DisplayServer, Box<dyn std::error::Error>> {
    match std::env::var("XDG_SESSION_TYPE")?.as_str() {
        "wayland" => Ok(DisplayServer::Wayland),
        "x11" => Ok(DisplayServer::X11),
        _ => Err("Unsupported display server.".into()),
    }
}

pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let output = match get_display_server()? {
        DisplayServer::Wayland => Command::new("wl-paste").output()?,
        DisplayServer::X11 => Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()?,
    };

    if !output.status.success() {
        return Err("Failed to read from clipboard.".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = match get_display_server()? {
        DisplayServer::Wayland => Command::new("wl-copy"),
        DisplayServer::X11 => {
            let mut command = Command::new("xclip");
            command.args(["-selection", "clipboard"]);
            command
        }
    };

    let mut child = command.stdin(Stdio::piped()).spawn()?;
    child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
    let status = child.wait()?;

    if !status.success() {
        return Err("Failed to write to clipboard.".into());
    }

    Ok(())
}
