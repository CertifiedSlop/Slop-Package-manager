//! Flake input management for NixOS
//!
//! Handles adding, removing, and updating flake inputs.

use anyhow::{bail, Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a flake input
#[derive(Debug, Clone)]
pub struct FlakeInput {
    pub name: String,
    pub url: String,
    pub follows: Option<String>,
}

/// Represents a flake.nix file
#[derive(Debug, Clone)]
pub struct Flake {
    pub path: PathBuf,
    pub content: String,
    pub description: Option<String>,
    pub inputs: Vec<FlakeInput>,
    pub outputs: String,
}

impl Flake {
    /// Load and parse a flake.nix file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read flake file: {:?}", path))?;

        let description = Self::extract_description(&content);
        let inputs = Self::extract_inputs(&content)?;
        let outputs = Self::extract_outputs(&content);

        Ok(Flake {
            path,
            content,
            description,
            inputs,
            outputs,
        })
    }

    /// Extract the description from the flake
    fn extract_description(content: &str) -> Option<String> {
        let re = Regex::new(r#"description\s*=\s*"([^"]+)""#).ok()?;
        re.captures(content)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Extract inputs from the flake
    fn extract_inputs(content: &str) -> Result<Vec<FlakeInput>> {
        let mut inputs = Vec::new();
        let mut seen_names = std::collections::HashSet::new();

        // Match inputs = { ... };
        let inputs_re = Regex::new(r"(?s)inputs\s*=\s*\{([^}]+)\}").ok()
            .context("Failed to compile regex")?;

        if let Some(caps) = inputs_re.captures(content) {
            let inputs_block = caps.get(1).unwrap().as_str();

            // Match complex inputs: name.url = "..." (name can contain hyphens)
            let complex_re = Regex::new(r#"([\w-]+)\.url\s*=\s*"([^"]+)""#).ok()
                .context("Failed to compile complex input regex")?;

            for caps in complex_re.captures_iter(inputs_block) {
                let name = caps.get(1).unwrap().as_str();
                let url = caps.get(2).unwrap().as_str();

                if seen_names.insert(name) {
                    inputs.push(FlakeInput {
                        name: name.to_string(),
                        url: url.to_string(),
                        follows: None,
                    });
                }
            }
        }

        Ok(inputs)
    }

    /// Extract outputs section
    fn extract_outputs(content: &str) -> String {
        let outputs_re = Regex::new(r"(?s)outputs\s*=\s*\{[^}]+\}:").ok();
        if let Some(re) = outputs_re {
            if let Some(caps) = re.captures(content) {
                return caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default();
            }
        }
        String::new()
    }

    /// Check if the flake has a specific input
    pub fn has_input(&self, name: &str) -> bool {
        self.inputs.iter().any(|input| input.name == name)
    }

    /// Add a new input to the flake
    pub fn add_input(&mut self, name: &str, url: &str) -> Result<()> {
        if self.has_input(name) {
            bail!("Input '{}' already exists", name);
        }

        // Find the inputs section and add the new input
        let inputs_re = Regex::new(r"(?s)(inputs\s*=\s*\{)([^}]*)(\})").ok()
            .context("Failed to compile regex")?;

        if let Some(caps) = inputs_re.captures(&self.content) {
            let prefix = caps.get(1).unwrap().as_str();
            let inputs_content = caps.get(2).unwrap().as_str();
            let suffix = caps.get(3).unwrap().as_str();

            let new_input = format!("    {}.url = \"{}\";\n", name, url);
            let new_inputs = format!("{}{}", inputs_content, new_input);

            let new_content = self.content.replace(
                &format!("{}{}{}", prefix, inputs_content, suffix),
                &format!("{}{}{}", prefix, new_inputs, suffix),
            );

            self.content = new_content;
            self.inputs.push(FlakeInput {
                name: name.to_string(),
                url: url.to_string(),
                follows: None,
            });
        } else {
            // No inputs section found, need to add one
            self.add_inputs_section(name, url)?;
        }

        Ok(())
    }

    /// Remove an input from the flake
    pub fn remove_input(&mut self, name: &str) -> Result<bool> {
        if !self.has_input(name) {
            return Ok(false);
        }

        // Remove the input line - complex format: name.url = "..."
        let pattern = format!(r#"(?m)\s*{}\.url\s*=\s*"[^"]+";?\s*"#, regex::escape(name));
        let input_re = Regex::new(&pattern).ok().context("Failed to compile regex")?;
        self.content = input_re.replace_all(&self.content, "").to_string();

        // Also remove simple format: name = "..."
        let simple_pattern = format!(r#"(?m)\s*{}\s*=\s*"[^"]+";?\s*"#, regex::escape(name));
        let simple_re = Regex::new(&simple_pattern).ok().context("Failed to compile regex")?;
        self.content = simple_re.replace_all(&self.content, "").to_string();

        // Also remove from follows references
        let follows_pattern = format!(r#"(?m)\s*{}\.follows\s*=\s*"[^"]+";?\s*"#, regex::escape(name));
        let follows_re = Regex::new(&follows_pattern).ok().context("Failed to compile regex")?;
        self.content = follows_re.replace_all(&self.content, "").to_string();

        self.inputs.retain(|input| input.name != name);

        Ok(true)
    }

    /// Update an input URL
    pub fn update_input(&mut self, name: &str, new_url: &str) -> Result<bool> {
        if !self.has_input(name) {
            return Ok(false);
        }

        let pattern = format!(r#"({}\.url\s*=\s*)"[^"]+""#, regex::escape(name));
        let input_re = Regex::new(&pattern).ok().context("Failed to compile regex")?;

        self.content = input_re
            .replace_all(&self.content, &format!("${{1}}\"{}\"", new_url))
            .to_string();

        if let Some(input) = self.inputs.iter_mut().find(|i| i.name == name) {
            input.url = new_url.to_string();
        }

        Ok(true)
    }

    /// Add inputs section if it doesn't exist
    fn add_inputs_section(&mut self, name: &str, url: &str) -> Result<()> {
        // Find the description line or the beginning of the file
        let insert_re = Regex::new(r#"(?m)^(description\s*=\s*"[^"]+";)"#).ok()
            .context("Failed to compile regex")?;

        let new_section = format!(
            "\n  inputs = {{\n    {}.url = \"{}\";\n  }};",
            name, url
        );

        if let Some(caps) = insert_re.captures(&self.content) {
            let insert_pos = caps.get(1).unwrap().end();
            let mut new_content = String::new();
            new_content.push_str(&self.content[..insert_pos]);
            new_content.push_str(&new_section);
            new_content.push_str(&self.content[insert_pos..]);
            self.content = new_content;
        } else {
            // Insert after the opening brace
            if let Some(pos) = self.content.find('{') {
                let mut new_content = String::new();
                new_content.push_str(&self.content[..pos + 1]);
                new_content.push_str(&new_section);
                new_content.push_str(&self.content[pos + 1..]);
                self.content = new_content;
            }
        }

        self.inputs.push(FlakeInput {
            name: name.to_string(),
            url: url.to_string(),
            follows: None,
        });

        Ok(())
    }

    /// Save the flake to disk
    pub fn save(&self) -> Result<()> {
        fs::write(&self.path, &self.content)
            .with_context(|| format!("Failed to save flake to {:?}", self.path))?;
        Ok(())
    }

    /// Get the default flake path
    pub fn default_path() -> PathBuf {
        PathBuf::from("flake.nix")
    }

    /// Check if a flake exists at the given path
    pub fn exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
}

/// Update all flake inputs to their latest versions
pub fn update_flake_inputs<P: AsRef<Path>>(flake_path: P, flake_registry: bool) -> Result<String> {
    let flake_path = flake_path.as_ref();

    if !flake_path.exists() {
        bail!("Flake file not found: {:?}", flake_path);
    }

    let parent_dir = flake_path.parent().unwrap_or(Path::new("."));

    // Run nix flake update
    let mut cmd = std::process::Command::new("nix");
    cmd.arg("flake").arg("update");

    if flake_registry {
        cmd.arg("--update-input").arg("*");
    }

    cmd.current_dir(parent_dir);

    let output = cmd
        .output()
        .context("Failed to run nix flake update")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to update flake: {}", stderr);
    }
}

/// Lock the flake inputs
pub fn lock_flake_inputs<P: AsRef<Path>>(flake_path: P) -> Result<String> {
    let flake_path = flake_path.as_ref();

    if !flake_path.exists() {
        bail!("Flake file not found: {:?}", flake_path);
    }

    let parent_dir = flake_path.parent().unwrap_or(Path::new("."));

    let output = std::process::Command::new("nix")
        .arg("flake")
        .arg("lock")
        .current_dir(parent_dir)
        .output()
        .context("Failed to run nix flake lock")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to lock flake: {}", stderr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const SAMPLE_FLAKE: &str = r#"{
  description = "My NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: { };
}
"#;

    fn create_test_flake(dir: &TempDir, content: &str) -> PathBuf {
        let flake_path = dir.path().join("flake.nix");
        fs::write(&flake_path, content).unwrap();
        flake_path
    }

    #[test]
    fn test_load_flake() {
        let dir = TempDir::new().unwrap();
        let flake_path = create_test_flake(&dir, SAMPLE_FLAKE);

        let flake = Flake::load(&flake_path).unwrap();

        assert_eq!(flake.description, Some("My NixOS configuration".to_string()));
        assert_eq!(flake.inputs.len(), 2);
        assert!(flake.has_input("nixpkgs"));
        assert!(flake.has_input("flake-utils"));
    }

    #[test]
    fn test_add_input() {
        let dir = TempDir::new().unwrap();
        let flake_path = create_test_flake(&dir, SAMPLE_FLAKE);

        let mut flake = Flake::load(&flake_path).unwrap();
        flake.add_input("home-manager", "github:nix-community/home-manager").unwrap();

        assert!(flake.has_input("home-manager"));
        assert_eq!(flake.inputs.len(), 3);
    }

    #[test]
    fn test_remove_input() {
        let dir = TempDir::new().unwrap();
        let flake_path = create_test_flake(&dir, SAMPLE_FLAKE);

        let mut flake = Flake::load(&flake_path).unwrap();
        let removed = flake.remove_input("flake-utils").unwrap();

        assert!(removed);
        assert!(!flake.has_input("flake-utils"));
    }

    #[test]
    fn test_update_input() {
        let dir = TempDir::new().unwrap();
        let flake_path = create_test_flake(&dir, SAMPLE_FLAKE);

        let mut flake = Flake::load(&flake_path).unwrap();
        let updated = flake.update_input("nixpkgs", "github:nixos/nixpkgs/nixos-23.11").unwrap();

        assert!(updated);
        assert!(flake.content.contains("github:nixos/nixpkgs/nixos-23.11"));
    }
}
