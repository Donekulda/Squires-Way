//! Cross-platform external URI/command launch (Irony `ProcessRunner.LaunchExternalCommand`).

use std::io;
use std::process::Command;

/// Opens a URI or command the same way Irony does: `xdg-open` on Linux, shell `start` on Windows, `open` on macOS.
pub fn launch_external_command(command: &str) -> io::Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(command).spawn()?.wait()?;
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", command])
            .spawn()?
            .wait()?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(command).spawn()?.wait()?;
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        let _ = command;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "launch_external_command: unsupported OS",
        ))
    }
}
