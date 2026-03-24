//! AI Context Awareness
//!
//! Provides context about the existing system configuration to the AI,
//! enabling smarter recommendations and avoiding redundant suggestions.

use crate::nix_config::NixConfig;
use crate::package_resolver::PackageResolver;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Represents the current state of the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemContext {
    /// Currently installed packages
    pub installed_packages: HashSet<String>,
    /// Package categories detected (e.g., "development", "media", "networking")
    pub detected_categories: HashSet<String>,
    /// Whether home-manager is configured
    pub has_home_manager: bool,
    /// Whether flakes are enabled
    pub uses_flakes: bool,
    /// Desktop environment if detected
    pub desktop_environment: Option<String>,
    /// Development languages in use
    pub dev_languages: HashSet<String>,
    /// Recently installed packages (last 10)
    pub recent_packages: Vec<String>,
}

/// AI Context manager
pub struct AiContext {
    context: SystemContext,
    config_path: Option<String>,
}

impl AiContext {
    /// Create a new AI context from the system configuration
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_path = config_path.as_ref().to_string_lossy().to_string();
        let context = Self::build_context(&config_path)?;

        Ok(AiContext {
            context,
            config_path: Some(config_path),
        })
    }

    /// Create a default context (for testing or when config is unavailable)
    pub fn default_context() -> Self {
        AiContext {
            context: SystemContext {
                installed_packages: HashSet::new(),
                detected_categories: HashSet::new(),
                has_home_manager: false,
                uses_flakes: false,
                desktop_environment: None,
                dev_languages: HashSet::new(),
                recent_packages: Vec::new(),
            },
            config_path: None,
        }
    }

    /// Build context from configuration
    fn build_context(config_path: &str) -> Result<SystemContext> {
        let config = match NixConfig::load(config_path) {
            Ok(c) => c,
            Err(_) => {
                // Return empty context if config can't be loaded
                return Ok(SystemContext {
                    installed_packages: HashSet::new(),
                    detected_categories: HashSet::new(),
                    has_home_manager: false,
                    uses_flakes: false,
                    desktop_environment: None,
                    dev_languages: HashSet::new(),
                    recent_packages: Vec::new(),
                });
            }
        };

        let installed_packages: HashSet<String> = config.packages.iter().cloned().collect();
        let detected_categories = Self::detect_categories(&installed_packages);
        let dev_languages = Self::detect_dev_languages(&installed_packages);
        let desktop_environment = Self::detect_desktop(&installed_packages);

        Ok(SystemContext {
            installed_packages,
            detected_categories,
            has_home_manager: config.content.contains("home-manager"),
            uses_flakes: config.uses_flakes,
            desktop_environment,
            dev_languages,
            recent_packages: config.packages.iter().rev().take(10).cloned().collect(),
        })
    }

    /// Detect package categories from installed packages
    fn detect_categories(packages: &HashSet<String>) -> HashSet<String> {
        let mut categories = HashSet::new();

        // Development tools
        if packages.iter().any(|p| {
            p.contains("rust")
                || p.contains("cargo")
                || p.contains("nodejs")
                || p.contains("python")
                || p.contains("go")
                || p.contains("gcc")
                || p.contains("clang")
        }) {
            categories.insert("development".to_string());
        }

        // Media
        if packages.iter().any(|p| {
            p.contains("vlc")
                || p.contains("mpv")
                || p.contains("spotify")
                || p.contains("ffmpeg")
                || p.contains("audacity")
        }) {
            categories.insert("media".to_string());
        }

        // Networking
        if packages.iter().any(|p| {
            p.contains("curl")
                || p.contains("wget")
                || p.contains("httpie")
                || p.contains("netcat")
        }) {
            categories.insert("networking".to_string());
        }

        // Terminal
        if packages.iter().any(|p| {
            p.contains("alacritty")
                || p.contains("kitty")
                || p.contains("wezterm")
                || p.contains("tmux")
                || p.contains("screen")
        }) {
            categories.insert("terminal".to_string());
        }

        // Desktop
        if packages.iter().any(|p| {
            p.contains("gnome")
                || p.contains("kde")
                || p.contains("xfce")
                || p.contains("i3")
                || p.contains("sway")
        }) {
            categories.insert("desktop".to_string());
        }

        // Editors
        if packages.iter().any(|p| {
            p.contains("vim")
                || p.contains("neovim")
                || p.contains("emacs")
                || p.contains("vscode")
                || p.contains("sublime")
        }) {
            categories.insert("editors".to_string());
        }

        // Browsers
        if packages.iter().any(|p| {
            p.contains("firefox")
                || p.contains("chrome")
                || p.contains("chromium")
                || p.contains("brave")
        }) {
            categories.insert("browsers".to_string());
        }

        categories
    }

    /// Detect programming languages in use
    fn detect_dev_languages(packages: &HashSet<String>) -> HashSet<String> {
        let mut languages = HashSet::new();

        if packages.iter().any(|p| p.contains("rust") || p.contains("cargo")) {
            languages.insert("rust".to_string());
        }
        if packages.iter().any(|p| p.contains("nodejs") || p.contains("npm") || p.contains("yarn"))
        {
            languages.insert("javascript".to_string());
            languages.insert("typescript".to_string());
        }
        if packages.iter().any(|p| p.contains("python") || p.contains("pip")) {
            languages.insert("python".to_string());
        }
        if packages.iter().any(|p| p.contains("go") || p.contains("golang")) {
            languages.insert("go".to_string());
        }
        if packages.iter().any(|p| p.contains("gcc") || p.contains("g++")) {
            languages.insert("c".to_string());
            languages.insert("cpp".to_string());
        }
        if packages.iter().any(|p| p.contains("clang")) {
            languages.insert("rust".to_string());
        }
        if packages.iter().any(|p| p.contains("jdk") || p.contains("openjdk")) {
            languages.insert("java".to_string());
        }

        languages
    }

    /// Detect desktop environment
    fn detect_desktop(packages: &HashSet<String>) -> Option<String> {
        if packages.iter().any(|p| p.contains("gnome")) {
            return Some("GNOME".to_string());
        }
        if packages.iter().any(|p| p.contains("plasma") || p.contains("kde")) {
            return Some("KDE Plasma".to_string());
        }
        if packages.iter().any(|p| p.contains("xfce")) {
            return Some("XFCE".to_string());
        }
        if packages.iter().any(|p| p.contains("i3") || p.contains("sway")) {
            return Some("Tiling WM".to_string());
        }
        None
    }

    /// Get the system context
    pub fn get_context(&self) -> &SystemContext {
        &self.context
    }

    /// Check if a package is already installed
    pub fn is_installed(&self, package: &str) -> bool {
        self.context.installed_packages.contains(package)
    }

    /// Get installed packages in a category
    pub fn get_packages_in_category(&self, category: &str) -> Vec<String> {
        match category {
            "development" => self
                .context
                .installed_packages
                .iter()
                .filter(|p| {
                    p.contains("rust")
                        || p.contains("cargo")
                        || p.contains("nodejs")
                        || p.contains("python")
                        || p.contains("go")
                })
                .cloned()
                .collect(),
            "media" => self
                .context
                .installed_packages
                .iter()
                .filter(|p| {
                    p.contains("vlc") || p.contains("mpv") || p.contains("spotify")
                })
                .cloned()
                .collect(),
            "editors" => self
                .context
                .installed_packages
                .iter()
                .filter(|p| {
                    p.contains("vim")
                        || p.contains("neovim")
                        || p.contains("emacs")
                        || p.contains("vscode")
                })
                .cloned()
                .collect(),
            "browsers" => self
                .context
                .installed_packages
                .iter()
                .filter(|p| {
                    p.contains("firefox")
                        || p.contains("chrome")
                        || p.contains("chromium")
                })
                .cloned()
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Check if the system has a category of packages
    pub fn has_category(&self, category: &str) -> bool {
        self.context.detected_categories.contains(category)
    }

    /// Get development languages
    pub fn get_dev_languages(&self) -> Vec<String> {
        self.context.dev_languages.iter().cloned().collect()
    }

    /// Generate a context summary for AI prompts
    pub fn to_prompt_context(&self) -> String {
        let mut parts = Vec::new();

        if !self.context.installed_packages.is_empty() {
            parts.push(format!(
                "Currently installed: {} packages",
                self.context.installed_packages.len()
            ));
        }

        if !self.context.dev_languages.is_empty() {
            parts.push(format!(
                "Development languages: {}",
                self.context.dev_languages.iter().cloned().collect::<Vec<_>>().join(", ")
            ));
        }

        if let Some(de) = &self.context.desktop_environment {
            parts.push(format!("Desktop: {}", de));
        }

        if self.context.has_home_manager {
            parts.push("Home-Manager: configured".to_string());
        }

        if self.context.uses_flakes {
            parts.push("Nix Flakes: enabled".to_string());
        }

        if parts.is_empty() {
            "No existing configuration detected".to_string()
        } else {
            parts.join(" | ")
        }
    }

    /// Suggest packages based on context
    pub fn suggest_packages(
        &self,
        resolver: &PackageResolver,
        intent: &str,
    ) -> Vec<(String, String)> {
        let mut suggestions = Vec::new();

        match intent {
            "development" => {
                // Suggest complementary dev tools
                if self.context.dev_languages.contains("rust")
                    && !self.context.installed_packages.contains("rust-analyzer")
                {
                    suggestions.push((
                        "rust-analyzer".to_string(),
                        "LSP server for Rust development".to_string(),
                    ));
                }
                if !self.context.dev_languages.contains("rust") {
                    suggestions.push((
                        "rustup".to_string(),
                        "Rust toolchain installer".to_string(),
                    ));
                }
                if !self.context.installed_packages.contains("git") {
                    suggestions
                        .push(("git".to_string(), "Version control system".to_string()));
                }
            }
            "desktop" => {
                // Suggest desktop utilities
                if !self.context.installed_packages.contains("dunst") {
                    suggestions.push(("dunst".to_string(), "Notification daemon".to_string()));
                }
                if !self.context.installed_packages.contains("picom") {
                    suggestions.push(("picom".to_string(), "Compositor for X11".to_string()));
                }
            }
            "media" => {
                if !self.context.installed_packages.contains("vlc") {
                    suggestions.push(("vlc".to_string(), "Media player".to_string()));
                }
                if !self.context.installed_packages.contains("ffmpeg") {
                    suggestions.push(("ffmpeg".to_string(), "Video processing tool".to_string()));
                }
            }
            _ => {}
        }

        // Filter out already installed packages
        suggestions.retain(|(pkg, _)| !self.context.installed_packages.contains(pkg));

        suggestions
    }

    /// Detect potential conflicts before installation
    pub fn detect_conflicts(&self, packages: &[String]) -> Vec<String> {
        let mut conflicts = Vec::new();

        for package in packages {
            // Check for conflicting packages
            if package.contains("vim") && self.context.installed_packages.contains("neovim") {
                conflicts.push(format!(
                    "Both vim and neovim will be installed. Consider using only one."
                ));
            }
            if package.contains("neovim") && self.context.installed_packages.contains("vim") {
                conflicts.push(format!(
                    "Both neovim and vim will be installed. Consider using only one."
                ));
            }
            if package.contains("firefox")
                && (self.context.installed_packages.contains("chromium")
                    || self.context.installed_packages.contains("chrome"))
            {
                conflicts.push(
                    "Multiple browsers will be installed. This is normal but worth noting."
                        .to_string(),
                );
            }
        }

        conflicts
    }

    /// Refresh the context
    pub fn refresh(&mut self) -> Result<()> {
        if let Some(ref path) = self.config_path {
            self.context = Self::build_context(path)?;
        }
        Ok(())
    }
}

/// AI suggestion with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub package: String,
    pub reason: String,
    pub confidence: f32,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config(content: &str) -> (String, TempDir) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("configuration.nix");
        fs::write(&path, content).unwrap();
        (path.to_string_lossy().to_string(), dir)
    }

    #[test]
    fn test_detect_categories() {
        let mut packages = HashSet::new();
        packages.insert("firefox".to_string());
        packages.insert("neovim".to_string());
        packages.insert("rustup".to_string());

        let categories = AiContext::detect_categories(&packages);

        assert!(categories.contains("development"));
        assert!(categories.contains("browsers"));
        assert!(categories.contains("editors"));
    }

    #[test]
    fn test_detect_dev_languages() {
        let mut packages = HashSet::new();
        packages.insert("rustup".to_string());
        packages.insert("nodejs".to_string());
        packages.insert("python3".to_string());

        let languages = AiContext::detect_dev_languages(&packages);

        assert!(languages.contains("rust"));
        assert!(languages.contains("javascript"));
        assert!(languages.contains("python"));
    }

    #[test]
    fn test_context_from_config() {
        let content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
    neovim
    git
    rustup
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let ctx = AiContext::new(&path).unwrap();

        assert!(ctx.is_installed("firefox"));
        assert!(ctx.is_installed("neovim"));
        assert!(ctx.has_category("development"));
        assert!(ctx.has_category("browsers"));
    }

    #[test]
    fn test_suggest_packages() {
        let content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    rustup
    cargo
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let ctx = AiContext::new(&path).unwrap();
        let resolver = PackageResolver::new();

        let suggestions = ctx.suggest_packages(&resolver, "development");

        // Should suggest rust-analyzer since we have rust but not rust-analyzer
        assert!(suggestions
            .iter()
            .any(|(pkg, _)| pkg == "rust-analyzer"));
    }
}
