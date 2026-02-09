use crate::config::{Compiler, DotfileEntry};
use color_eyre::Result;
use std::process::Command;

pub struct InstallResult {
    pub success: bool,
    pub output: String,
}

pub fn install_dotfile(entry: &DotfileEntry) -> Result<InstallResult> {
    let output = match &entry.config.compiler {
        Compiler::Gcc { flags } => {
            let mut cmd = Command::new("gcc");
            if let Some(flags) = flags {
                cmd.args(flags);
            }
            cmd.current_dir(&entry.path);
            cmd.output()?
        }
        Compiler::Make { target } => {
            // Try to run install.sh directly if it exists, otherwise use make
            let install_script = entry.path.join("install.sh");
            if install_script.exists() {
                let mut cmd = Command::new("bash");
                cmd.arg("install.sh");
                cmd.current_dir(&entry.path);
                cmd.output()?
            } else {
                let mut cmd = Command::new("make");
                if let Some(target) = target {
                    cmd.arg(target);
                } else {
                    cmd.arg("install");
                }
                cmd.current_dir(&entry.path);
                cmd.output()?
            }
        }
        Compiler::Cargo { release } => {
            let mut cmd = Command::new("cargo");
            cmd.arg("build");
            if release.unwrap_or(false) {
                cmd.arg("--release");
            }
            cmd.current_dir(&entry.path);
            cmd.output()?
        }
        Compiler::Nix { flake } => {
            let mut cmd = Command::new("nix-build");
            if flake.unwrap_or(false) {
                cmd.arg("--flake");
            }
            cmd.current_dir(&entry.path);
            cmd.output()?
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}\n{}", stdout, stderr);

    Ok(InstallResult {
        success: output.status.success(),
        output: combined.trim().to_string(),
    })
}
