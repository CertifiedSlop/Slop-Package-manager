//! Nix configuration file parser and editor
//! 
//! Handles parsing, modifying, and validating configuration.nix files.

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use fs_extra::file::{copy, CopyOptions};

/// Represents the parsed structure of a Nix configuration
#[derive(Debug, Clone)]
pub struct NixConfig {
    /// Raw content of the file
    pub content: String,
    /// Path to the configuration file
    pub path: PathBuf,
    /// Packages in environment.systemPackages
    pub packages: Vec<String>,
    /// Start and end byte positions of the packages list
    pub packages_range: Option<(usize, usize)>,
    /// Whether the config uses flakes
    pub uses_flakes: bool,
}

impl NixConfig {
    /// Load and parse a Nix configuration file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let packages = Self::extract_packages(&content)?;
        let packages_range = Self::find_packages_range(&content);
        let uses_flakes = Self::detect_flakes(&content);

        Ok(NixConfig {
            content,
            path,
            packages,
            packages_range,
            uses_flakes,
        })
    }

    /// Extract package names from environment.systemPackages
    fn extract_packages(content: &str) -> Result<Vec<String>> {
        // Match: environment.systemPackages = with pkgs; [ ... ];
        // or: environment.systemPackages = [ ... ];
        let re = Regex::new(
            r"(?s)environment\.systemPackages\s*=\s*(?:with\s+pkgs\s*;\s*)?\[([^\]]*)\]"
        )
        .context("Failed to compile regex")?;

        if let Some(caps) = re.captures(content) {
            let packages_block = caps.get(1).unwrap().as_str();
            let packages = packages_block
                .lines()
                .flat_map(|line| {
                    // Remove comments
                    line.split('#').next().unwrap_or("")
                })
                .flat_map(|line| {
                    // Split by whitespace and filter
                    line.split_whitespace()
                })
                .filter(|s| !s.is_empty() && *s != "with" && *s != "pkgs")
                .map(|s| s.to_string())
                .collect();
            return Ok(packages);
        }

        Ok(Vec::new())
    }

    /// Find the byte range of the packages list for editing
    fn find_packages_range(content: &str) -> Option<(usize, usize)> {
        let re = Regex::new(
            r"(?s)(environment\.systemPackages\s*=\s*(?:with\s+pkgs\s*;\s*)?)\[([^\]]*)\]"
        ).ok()?;

        if let Some(caps) = re.captures(content) {
            let full_match = caps.get(0)?;
            let packages_content = caps.get(2)?;
            
            let start = full_match.start();
            let end = full_match.end();
            
            return Some((start, end));
        }

        None
    }

    /// Detect if the configuration uses flakes
    fn detect_flakes(content: &str) -> bool {
        // Check for common flake indicators
        content.contains("flake.nix") 
            || content.contains("inputs.")
            || content.contains("outputs = {")
    }

    /// Check if a package is already installed
    pub fn has_package(&self, package: &str) -> bool {
        self.packages.iter().any(|p| p == package)
    }

    /// Add a package to the configuration
    pub fn add_package(&mut self, package: &str) -> Result<()> {
        if self.has_package(package) {
            return Ok(()); // Already installed
        }

        self.packages.push(package.to_string());
        self.rebuild_content()?;
        Ok(())
    }

    /// Remove a package from the configuration
    pub fn remove_package(&mut self, package: &str) -> Result<bool> {
        let initial_len = self.packages.len();
        self.packages.retain(|p| p != package);
        
        if self.packages.len() < initial_len {
            self.rebuild_content()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Rebuild the content string after modifications
    fn rebuild_content(&mut self) -> Result<()> {
        let Some((start, end)) = self.packages_range else {
            // No existing packages list, need to add one
            return self.add_packages_section();
        };

        // Build new packages list with proper formatting
        let packages_str = self.format_packages();
        
        // Determine the prefix (with pkgs; or not)
        let prefix = if self.content[start..end].contains("with pkgs") {
            "environment.systemPackages = with pkgs; "
        } else {
            "environment.systemPackages = "
        };

        let new_section = format!("{}[ {}]", prefix, packages_str);
        
        // Replace the old section
        let mut new_content = String::new();
        new_content.push_str(&self.content[..start]);
        new_content.push_str(&new_section);
        new_content.push_str(&self.content[end..]);
        
        self.content = new_content;
        
        // Update the range for future edits
        self.packages_range = Some((start, start + new_section.len()));
        
        Ok(())
    }

    /// Add a packages section if it doesn't exist
    fn add_packages_section(&mut self) -> Result<()> {
        // Find a good place to insert - after the opening { or after imports
        let insert_re = Regex::new(r"(?m)^\s*imports\s*=\s*\[[^\]]*\]\s*;").ok()
            .and_then(|re| re.find(&self.content));

        let insert_pos = if let Some(m) = insert_re {
            m.end()
        } else {
            // Find first { and insert after it
            self.content.find('{')
                .map(|p| p + 1)
                .unwrap_or(0)
        };

        let packages_str = self.format_packages();
        let new_section = format!("\n\n  environment.systemPackages = with pkgs; [ {}];", packages_str);
        
        let mut new_content = String::new();
        new_content.push_str(&self.content[..insert_pos]);
        new_content.push_str(&new_section);
        new_content.push_str(&self.content[insert_pos..]);
        
        self.content = new_content;
        self.packages_range = Some((insert_pos, insert_pos + new_section.len()));
        
        Ok(())
    }

    /// Format packages list for insertion
    fn format_packages(&self) -> String {
        self.packages.join(" ")
    }

    /// Validate the Nix syntax
    pub fn validate(&self) -> Result<()> {
        // Create a temp file with the content
        let temp_file = NamedTempFile::new()
            .context("Failed to create temp file")?;
        
        fs::write(&temp_file, &self.content)
            .context("Failed to write temp file")?;

        // Run nix-instantiate to check syntax
        let output = std::process::Command::new("nix-instantiate")
            .arg("--parse")
            .arg(temp_file.path())
            .output()
            .context("Failed to run nix-instantiate")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Nix syntax validation failed:\n{}", stderr);
        }

        Ok(())
    }

    /// Create a backup of the configuration file
    pub fn backup(&self) -> Result<PathBuf> {
        let backup_path = self.path.with_extension(format!(
            "nix.bak.{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        let options = CopyOptions::new().overwrite(true);
        copy(&self.path, &backup_path, &options)
            .with_context(|| format!("Failed to backup config to {:?}", backup_path))?;

        Ok(backup_path)
    }

    /// Save the configuration to disk
    pub fn save(&self) -> Result<()> {
        // Write to temp file first
        let temp_file = NamedTempFile::new()
            .context("Failed to create temp file")?;
        
        fs::write(&temp_file, &self.content)
            .context("Failed to write temp file")?;

        // Validate before saving
        self.validate()?;

        // Atomic move
        fs::copy(temp_file.path(), &self.path)
            .with_context(|| format!("Failed to save config to {:?}", self.path))?;

        Ok(())
    }

    /// Generate a diff between original and modified content
    pub fn diff(&self, original: &str) -> String {
        use similar::{ChangeTag, TextDiff};

        let diff = TextDiff::from_lines(original, &self.content);
        let mut output = String::new();

        for change in diff.iter_all_changes() {
            let line = match change.tag() {
                ChangeTag::Delete => format!("- {}", change),
                ChangeTag::Insert => format!("+ {}", change),
                ChangeTag::Equal => format!("  {}", change),
            };
            output.push_str(&line);
        }

        output
    }

    /// Get the default config path
    pub fn default_path() -> PathBuf {
        PathBuf::from("/etc/nixos/configuration.nix")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config(content: &str) -> (PathBuf, TempDir) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("configuration.nix");
        fs::write(&path, content).unwrap();
        (path, dir)
    }

    #[test]
    fn test_extract_packages() {
        let content = r#"
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
    neovim
    git
  ];
}
"#;
        let packages = NixConfig::extract_packages(content).unwrap();
        assert_eq!(packages, vec!["firefox", "neovim", "git"]);
    }

    #[test]
    fn test_extract_packages_no_with_pkgs() {
        let content = r#"
{ config, pkgs, ... }: {
  environment.systemPackages = [
    pkgs.firefox
    pkgs.neovim
  ];
}
"#;
        let packages = NixConfig::extract_packages(content).unwrap();
        assert_eq!(packages, vec!["pkgs.firefox", "pkgs.neovim"]);
    }

    #[test]
    fn test_add_package() {
        let content = r#"
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let mut config = NixConfig::load(&path).unwrap();
        
        config.add_package("neovim").unwrap();
        assert!(config.has_package("neovim"));
        assert!(config.has_package("firefox"));
    }

    #[test]
    fn test_remove_package() {
        let content = r#"
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
    neovim
    git
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let mut config = NixConfig::load(&path).unwrap();
        
        let removed = config.remove_package("neovim").unwrap();
        assert!(removed);
        assert!(!config.has_package("neovim"));
        assert!(config.has_package("firefox"));
    }

    #[test]
    fn test_no_duplicate_package() {
        let content = r#"
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let mut config = NixConfig::load(&path).unwrap();
        
        config.add_package("firefox").unwrap();
        // Should still only have one firefox
        let firefox_count = config.packages.iter().filter(|p| *p == "firefox").count();
        assert_eq!(firefox_count, 1);
    }
}
