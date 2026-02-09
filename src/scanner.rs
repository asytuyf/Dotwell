use crate::config::{DotfileConfig, DotfileEntry};
use color_eyre::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct DotfileScanner {
    search_paths: Vec<PathBuf>,
}

impl DotfileScanner {
    pub fn new() -> Self {
        let mut search_paths = vec![];

        // Add common dotfile locations
        if let Some(home) = dirs::home_dir() {
            search_paths.push(home.join(".config"));
            search_paths.push(home.join("dotfiles"));
            search_paths.push(home.join(".dotfiles"));
        }

        // Add NixOS configuration directory
        let nixos_path = std::path::PathBuf::from("/etc/nixos");
        if nixos_path.exists() {
            search_paths.push(nixos_path);
        }

        // Add current directory
        if let Ok(current) = std::env::current_dir() {
            search_paths.push(current);
        }

        Self { search_paths }
    }

    #[allow(dead_code)]
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self { search_paths: paths }
    }

    pub fn scan(&self) -> Result<Vec<DotfileEntry>> {
        let mut entries = vec![];

        for search_path in &self.search_paths {
            if !search_path.exists() {
                continue;
            }

            self.scan_directory(search_path, &mut entries)?;
        }

        Ok(entries)
    }

    fn scan_directory(&self, dir: &Path, entries: &mut Vec<DotfileEntry>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        // Look for dotwell.toml or dotwell.json
        let toml_path = dir.join("dotwell.toml");
        let json_path = dir.join("dotwell.json");

        if toml_path.exists() {
            if let Ok(config) = DotfileConfig::from_toml(&toml_path) {
                entries.push(DotfileEntry {
                    config,
                    path: dir.to_path_buf(),
                });
            }
        } else if json_path.exists() {
            if let Ok(config) = DotfileConfig::from_json(&json_path) {
                entries.push(DotfileEntry {
                    config,
                    path: dir.to_path_buf(),
                });
            }
        }

        // Recursively scan subdirectories
        if let Ok(read_dir) = fs::read_dir(dir) {
            for entry in read_dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let path = entry.path();
                        // Skip hidden directories and common ignore patterns
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.')
                                && name != "target"
                                && name != "node_modules"
                                && name != "build" {
                                self.scan_directory(&path, entries)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
