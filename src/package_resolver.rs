//! Package resolver for Nix packages
//!
//! Handles package name resolution, searching, and validation.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Package information from nixpkgs
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct PackageInfo {
    pub name: String,
    pub pname: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}

/// Package search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub attr_name: String,
    pub package: PackageInfo,
    pub score: f32,
}

/// Package resolver that can query nixpkgs
#[derive(Clone)]
pub struct PackageResolver {
    /// Cache of known package mappings (natural name -> attr_name)
    package_aliases: HashMap<&'static str, &'static str>,
}

impl PackageResolver {
    pub fn new() -> Self {
        let mut resolver = PackageResolver {
            package_aliases: HashMap::new(),
        };
        resolver.init_aliases();
        resolver
    }

    /// Initialize common package aliases
    fn init_aliases(&mut self) {
        // Browser aliases
        self.package_aliases.insert("firefox", "firefox");
        self.package_aliases.insert("chrome", "google-chrome");
        self.package_aliases.insert("chromium", "chromium");
        self.package_aliases.insert("brave", "brave");
        self.package_aliases.insert("browser", "firefox"); // default

        // Editor aliases
        self.package_aliases.insert("neovim", "neovim");
        self.package_aliases.insert("nvim", "neovim");
        self.package_aliases.insert("vim", "vim");
        self.package_aliases.insert("emacs", "emacs");
        self.package_aliases.insert("vscode", "vscode");
        self.package_aliases.insert("code", "vscode");
        self.package_aliases.insert("editor", "neovim"); // default
        self.package_aliases.insert("text editor", "neovim");

        // Terminal aliases
        self.package_aliases.insert("terminal", "alacritty");
        self.package_aliases.insert("term", "alacritty");
        self.package_aliases.insert("alacritty", "alacritty");
        self.package_aliases.insert("kitty", "kitty");
        self.package_aliases.insert("wezterm", "wezterm");
        self.package_aliases.insert("foot", "foot");

        // Shell aliases
        self.package_aliases.insert("zsh", "zsh");
        self.package_aliases.insert("fish", "fish");
        self.package_aliases.insert("bash", "bash");
        self.package_aliases.insert("nushell", "nushell");
        self.package_aliases.insert("shell", "zsh");

        // Git aliases
        self.package_aliases.insert("git", "git");
        self.package_aliases.insert("gitui", "gitui");
        self.package_aliases.insert("lazygit", "lazygit");

        // Development tools
        self.package_aliases.insert("rust", "rustup");
        self.package_aliases.insert("cargo", "rustup");
        self.package_aliases.insert("rustc", "rustup");
        self.package_aliases.insert("python", "python3");
        self.package_aliases.insert("python3", "python3");
        self.package_aliases.insert("node", "nodejs");
        self.package_aliases.insert("nodejs", "nodejs");
        self.package_aliases.insert("go", "go");
        self.package_aliases.insert("golang", "go");

        // Utilities
        self.package_aliases.insert("htop", "htop");
        self.package_aliases.insert("btop", "btop");
        self.package_aliases.insert("top", "htop");
        self.package_aliases.insert("tree", "tree");
        self.package_aliases.insert("ripgrep", "ripgrep");
        self.package_aliases.insert("rg", "ripgrep");
        self.package_aliases.insert("fd", "fd");
        self.package_aliases.insert("fzf", "fzf");
        self.package_aliases.insert("bat", "bat");
        self.package_aliases.insert("eza", "eza");
        self.package_aliases.insert("ls", "eza");

        // Media
        self.package_aliases.insert("vlc", "vlc");
        self.package_aliases.insert("mpv", "mpv");
        self.package_aliases.insert("player", "mpv");
        self.package_aliases.insert("spotify", "spotify");

        // Communication
        self.package_aliases.insert("discord", "discord");
        self.package_aliases.insert("telegram", "telegram-desktop");
        self.package_aliases.insert("signal", "signal-desktop");
        self.package_aliases.insert("slack", "slack");

        // File managers
        self.package_aliases.insert("ranger", "ranger");
        self.package_aliases.insert("nnn", "nnn");
        self.package_aliases.insert("lf", "lf");
        self.package_aliases.insert("file manager", "ranger");

        // System tools
        self.package_aliases.insert("docker", "docker");
        self.package_aliases.insert("podman", "podman");
        self.package_aliases.insert("tmux", "tmux");
        self.package_aliases.insert("screen", "screen");
    }

    /// Resolve a natural name to a nixpkgs attribute name
    pub fn resolve<'a>(&self, name: &'a str) -> Option<&'a str> {
        let name_lower = name.to_lowercase();

        // Check direct alias
        if let Some(&attr) = self.package_aliases.get(name_lower.as_str()) {
            return Some(attr);
        }

        // Return the name as-is if no alias found
        // The user might be providing the exact attr_name
        Some(name)
    }

    /// Search for packages matching a query
    /// This uses nix-locate if available, otherwise falls back to alias matching
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // First, check aliases for matches
        for (&alias, &attr) in &self.package_aliases {
            if alias.contains(&query_lower) || attr.contains(&query_lower) {
                results.push(SearchResult {
                    attr_name: attr.to_string(),
                    package: PackageInfo {
                        name: alias.to_string(),
                        pname: Some(attr.to_string()),
                        version: None,
                        description: Some(format!("Alias: {} -> {}", alias, attr)),
                        license: None,
                        homepage: None,
                    },
                    score: 1.0,
                });
            }
        }

        // Try to use nix-locate for more comprehensive search
        if let Ok(nix_results) = self.search_with_nix_locate(&query_lower) {
            for result in nix_results {
                if !results.iter().any(|r| r.attr_name == result.attr_name) {
                    results.push(result);
                }
            }
        }

        // Sort by score
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Search using nix-locate command
    fn search_with_nix_locate(&self, query: &str) -> Result<Vec<SearchResult>> {
        let output = std::process::Command::new("nix-locate")
            .args([
                "--minimal",
                "--no-group",
                "--type",
                "x",
                "--whole-name",
                "--at-root",
                query,
            ])
            .output()
            .context("Failed to run nix-locate")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();

        for line in stdout.lines().take(20) {
            // nix-locate output format: attr.path output/path/to/file
            if let Some(attr) = line.split_whitespace().next() {
                // Extract just the package name from attr.path
                let pkg_name = attr.split('.').next_back().unwrap_or(attr);

                results.push(SearchResult {
                    attr_name: attr.to_string(),
                    package: PackageInfo {
                        name: pkg_name.to_string(),
                        pname: Some(attr.to_string()),
                        version: None,
                        description: None,
                        license: None,
                        homepage: None,
                    },
                    score: 0.5,
                });
            }
        }

        Ok(results)
    }

    /// Validate that a package exists in nixpkgs
    pub fn validate_package(&self, package: &str) -> Result<bool> {
        // Try nix-env query to check if package exists
        let output = std::process::Command::new("nix-env")
            .args(["-qaP", "--out-path", "--xml"])
            .arg(package)
            .output();

        match output {
            Ok(out) => Ok(out.status.success() && !out.stdout.is_empty()),
            Err(_) => {
                // If nix-env fails, assume the package is valid
                // This allows offline usage
                Ok(true)
            }
        }
    }

    /// Get package suggestions for a potentially misspelled package
    pub fn suggest(&self, package: &str) -> Vec<String> {
        let package_lower = package.to_lowercase();
        let mut suggestions: Vec<_> = self
            .package_aliases
            .keys()
            .filter(|&k| {
                // Simple Levenshtein-like suggestion: check if names are similar
                let k_lower = k.to_lowercase();
                k_lower.contains(&package_lower) || package_lower.contains(&k_lower)
            })
            .map(|&s| s.to_string())
            .collect();

        suggestions.truncate(5);
        suggestions
    }
}

impl Default for PackageResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_firefox() {
        let resolver = PackageResolver::new();
        assert_eq!(resolver.resolve("firefox"), Some("firefox"));
    }

    #[test]
    fn test_resolve_alias() {
        let resolver = PackageResolver::new();
        assert_eq!(resolver.resolve("nvim"), Some("neovim"));
        assert_eq!(resolver.resolve("browser"), Some("firefox"));
    }

    #[test]
    fn test_search() {
        let resolver = PackageResolver::new();
        let results = resolver.search("editor");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.attr_name == "neovim"));
    }

    #[test]
    fn test_suggest() {
        let resolver = PackageResolver::new();
        let suggestions = resolver.suggest("firef");
        assert!(suggestions.contains(&"firefox".to_string()));
    }
}
