use std::io::Write;
use std::process::{Command, Stdio};

enum DisplayServer {
    Wayland,
    X11,
}

fn get_display_server() -> Result<DisplayServer, Box<dyn std::error::Error>> {
    let session = match std::env::var("XDG_SESSION_TYPE") {
        Ok(session) => session,
        Err(_) => return Err("Cannot find session backend.".into()),
    };

    match session.as_str() {
        "wayland" => Ok(DisplayServer::Wayland),
        "x11" => Ok(DisplayServer::X11),
        _ => Err("Unsupported display server.".into()),
    }
}

pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let display_server = get_display_server()?;
    let clipboard_command = match display_server {
        DisplayServer::Wayland => "wl-paste",
        DisplayServer::X11 => "xclip",
    };

    let output = match display_server {
        DisplayServer::Wayland => Command::new("wl-paste").output(),
        DisplayServer::X11 => Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output(),
    };

    let output = match output {
        Ok(output) => output,
        Err(_) => return Err(format!("Failed to run {clipboard_command}.").into()),
    };

    if !output.status.success() {
        return Err(format!("Failed to read clipboard from {clipboard_command}.").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let display_server = get_display_server()?;
    let clipboard_command = match display_server {
        DisplayServer::Wayland => "wl-copy",
        DisplayServer::X11 => "xclip",
    };

    let mut command = match display_server {
        DisplayServer::Wayland => Command::new("wl-copy"),
        DisplayServer::X11 => {
            let mut command = Command::new("xclip");
            command.args(["-selection", "clipboard"]);
            command
        }
    };

    let child = command.stdin(Stdio::piped()).spawn();

    let mut child = match child {
        Ok(child) => child,
        Err(_) => return Err(format!("Failed to run {clipboard_command}.").into()),
    };

    child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
    let status = child.wait()?;

    if !status.success() {
        return Err(format!("Failed to write clipboard to {clipboard_command}.").into());
    }

    Ok(())
}
