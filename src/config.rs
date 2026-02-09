use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotfileConfig {
    pub name: String,
    pub description: String,
    pub category: String,
    pub compiler: Compiler,
    pub dependencies: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Compiler {
    #[serde(rename = "gcc")]
    Gcc { flags: Option<Vec<String>> },
    #[serde(rename = "make")]
    Make { target: Option<String> },
    #[serde(rename = "cargo")]
    Cargo { release: Option<bool> },
    #[serde(rename = "nix")]
    Nix { flake: Option<bool> },
}

#[derive(Debug, Clone)]
pub struct DotfileEntry {
    pub config: DotfileConfig,
    pub path: PathBuf,
}

impl DotfileConfig {
    pub fn from_toml(path: &PathBuf) -> color_eyre::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DotfileConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_json(path: &PathBuf) -> color_eyre::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DotfileConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

pub fn get_compiler_name(compiler: &Compiler) -> &'static str {
    match compiler {
        Compiler::Gcc { .. } => "gcc",
        Compiler::Make { .. } => "make",
        Compiler::Cargo { .. } => "cargo",
        Compiler::Nix { .. } => "nix",
    }
}
