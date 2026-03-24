//! AI Configuration Optimizer
//!
//! Analyzes NixOS configurations and provides optimization suggestions.

use crate::nix_config::NixConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub title: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub category: Category,
    pub action: Action,
    pub estimated_savings: Option<String>,
}

/// Impact level of an optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
}

/// Category of optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Category {
    UnusedPackages,
    RedundantModules,
    BuildOptimization,
    Security,
    Performance,
    Cleanup,
}

/// Action to take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    RemovePackages(Vec<String>),
    DisableModules(Vec<String>),
    EnableSettings(Vec<(String, String)>),
    AddOverlays(Vec<String>),
    General(String),
}

/// Configuration optimizer
pub struct ConfigOptimizer {
    config: NixConfig,
}

impl ConfigOptimizer {
    /// Create a new optimizer from a config path
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config = NixConfig::load(config_path)?;
        Ok(ConfigOptimizer { config })
    }

    /// Analyze configuration and return optimization suggestions
    pub fn analyze(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Check for unused packages
        suggestions.extend(self.check_unused_packages());

        // Check for redundant modules
        suggestions.extend(self.check_redundant_modules());

        // Check for build optimizations
        suggestions.extend(self.check_build_optimizations());

        // Check for security improvements
        suggestions.extend(self.check_security());

        // Check for performance improvements
        suggestions.extend(self.check_performance());

        suggestions
    }

    /// Check for potentially unused packages
    fn check_unused_packages(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Common packages that are often installed but unused
        let _potentially_unused = [
            "vim",
            "neovim",
            "emacs", // Multiple editors
            "firefox",
            "chromium",
            "google-chrome", // Multiple browsers
            "vlc",
            "mpv", // Multiple media players
            "htop",
            "btop",
            "top", // Multiple system monitors
        ];

        let installed: HashSet<&String> = self.config.packages.iter().collect();
        let mut unused = Vec::new();

        // Check for multiple editors
        let editors: Vec<String> = ["vim", "neovim", "emacs"]
            .iter()
            .filter(|e| installed.iter().any(|p| p.contains(*e)))
            .map(|s| s.to_string())
            .collect();

        if editors.len() > 1 {
            unused.push(format!(
                "Multiple editors installed: {}. Consider keeping only one.",
                editors.join(", ")
            ));
        }

        // Check for multiple browsers
        let browsers: Vec<String> = ["firefox", "chromium", "google-chrome", "brave"]
            .iter()
            .filter(|b| installed.iter().any(|p| p.contains(*b)))
            .map(|s| s.to_string())
            .collect();

        if browsers.len() > 2 {
            unused.push(format!(
                "Multiple browsers installed: {}. Consider removing unused ones.",
                browsers.join(", ")
            ));
        }

        // Check for multiple media players
        let players: Vec<String> = ["vlc", "mpv", "celluloid"]
            .iter()
            .filter(|p| installed.iter().any(|pkg| pkg.contains(*p)))
            .map(|s| s.to_string())
            .collect();

        if players.len() > 1 {
            unused.push(format!(
                "Multiple media players: {}. Consider keeping only one.",
                players.join(", ")
            ));
        }

        if !unused.is_empty() {
            suggestions.push(OptimizationSuggestion {
                title: "Review potentially unused packages".to_string(),
                description: unused.join("\n"),
                impact: ImpactLevel::Low,
                category: Category::UnusedPackages,
                action: Action::General("Review and remove unused packages".to_string()),
                estimated_savings: Some("100MB - 500MB".to_string()),
            });
        }

        suggestions
    }

    /// Check for redundant module imports
    fn check_redundant_modules(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();
        let content = &self.config.content;

        // Check for common redundant patterns
        let redundant_patterns = [
            ("services.xserver.enable", "services.xserver.displayManager"),
            ("hardware.opengl", "hardware.opengl.driSupport"),
            ("networking.networkmanager.enable", "networking.wireless"),
        ];

        let mut found_redundant = Vec::new();

        for (parent, child) in redundant_patterns {
            if content.contains(parent) && content.contains(child) {
                found_redundant.push(format!("{} may be redundant with {}", child, parent));
            }
        }

        if !found_redundant.is_empty() {
            suggestions.push(OptimizationSuggestion {
                title: "Check for redundant module settings".to_string(),
                description: found_redundant.join("\n"),
                impact: ImpactLevel::Low,
                category: Category::RedundantModules,
                action: Action::General("Review and consolidate module settings".to_string()),
                estimated_savings: None,
            });
        }

        suggestions
    }

    /// Check for build optimization opportunities
    fn check_build_optimizations(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();
        let content = &self.config.content;

        // Check for nix-optimization settings
        let optimizations = [
            (
                "nix.settings.auto-optimise-store",
                "Enable automatic store optimization",
                "nix.settings.auto-optimise-store = true;",
            ),
            (
                "nix.settings.experimental-features",
                "Enable flakes and nix-command",
                "nix.settings.experimental-features = [ \"nix-command\" \"flakes\" ];",
            ),
            (
                "nix.settings.trusted-users",
                "Add user to trusted users for better performance",
                "nix.settings.trusted-users = [ \"root\" \"@wheel\" ];",
            ),
        ];

        let mut missing = Vec::new();

        for (setting, description, nix_code) in optimizations {
            if !content.contains(setting) && !content.contains(&setting.replace('.', "-")) {
                missing.push((description.to_string(), nix_code.to_string()));
            }
        }

        if !missing.is_empty() {
            let settings: Vec<(String, String)> = missing
                .iter()
                .map(|(_, code)| {
                    // Extract setting name and value
                    let parts: Vec<&str> = code.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        (
                            parts[0].trim().to_string(),
                            parts[1].trim().trim_end_matches(';').to_string(),
                        )
                    } else {
                        (code.clone(), String::new())
                    }
                })
                .collect();

            suggestions.push(OptimizationSuggestion {
                title: "Enable build optimizations".to_string(),
                description: missing
                    .iter()
                    .map(|(d, _)| d.as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
                impact: ImpactLevel::Medium,
                category: Category::BuildOptimization,
                action: Action::EnableSettings(settings),
                estimated_savings: Some("10-30% faster builds".to_string()),
            });
        }

        suggestions
    }

    /// Check for security improvements
    fn check_security(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();
        let content = &self.config.content;

        // Check for security-related settings
        let security_checks = [
            (
                "security.sudo",
                "Sudo configuration",
                "Ensure sudo is properly configured",
            ),
            (
                "security.pam",
                "PAM configuration",
                "Review PAM settings for authentication",
            ),
            (
                "networking.firewall",
                "Firewall configuration",
                "Consider enabling the firewall",
            ),
        ];

        let mut missing = Vec::new();

        for (setting, name, description) in security_checks {
            if !content.contains(setting) {
                missing.push(format!("{}: {}", name, description));
            }
        }

        if !missing.is_empty() {
            suggestions.push(OptimizationSuggestion {
                title: "Review security settings".to_string(),
                description: missing.join("\n"),
                impact: ImpactLevel::High,
                category: Category::Security,
                action: Action::General("Review and configure security settings".to_string()),
                estimated_savings: None,
            });
        }

        suggestions
    }

    /// Check for performance improvements
    fn check_performance(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();
        let content = &self.config.content;

        // Check for performance-related settings
        if !content.contains("zram") && !content.contains("zswap") {
            suggestions.push(OptimizationSuggestion {
                title: "Consider enabling ZRAM or Zswap".to_string(),
                description: "ZRAM/Zswap can improve memory performance and reduce swap usage."
                    .to_string(),
                impact: ImpactLevel::Medium,
                category: Category::Performance,
                action: Action::EnableSettings(vec![(
                    "zramSwap.enable".to_string(),
                    "true".to_string(),
                )]),
                estimated_savings: Some("Better memory utilization".to_string()),
            });
        }

        // Check for CPU microcode
        if !content.contains("hardware.cpu") {
            suggestions.push(OptimizationSuggestion {
                title: "Enable CPU microcode updates".to_string(),
                description: "CPU microcode updates provide security and stability improvements."
                    .to_string(),
                impact: ImpactLevel::High,
                category: Category::Performance,
                action: Action::General(
                    "Add hardware.cpu.intel.updateMicrocode or hardware.cpu.amd.updateMicrocode"
                        .to_string(),
                ),
                estimated_savings: Some("Security and stability".to_string()),
            });
        }

        suggestions
    }

    /// Generate a summary report
    pub fn generate_report(&self) -> OptimizationReport {
        let suggestions = self.analyze();

        let mut report = OptimizationReport {
            total_suggestions: suggestions.len(),
            high_impact: 0,
            medium_impact: 0,
            low_impact: 0,
            suggestions,
        };

        for suggestion in &report.suggestions {
            match suggestion.impact {
                ImpactLevel::High => report.high_impact += 1,
                ImpactLevel::Medium => report.medium_impact += 1,
                ImpactLevel::Low => report.low_impact += 1,
            }
        }

        report
    }
}

/// Optimization report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationReport {
    pub total_suggestions: usize,
    pub high_impact: usize,
    pub medium_impact: usize,
    pub low_impact: usize,
    pub suggestions: Vec<OptimizationSuggestion>,
}

impl OptimizationReport {
    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "Found {} optimization(s): {} high impact, {} medium impact, {} low impact",
            self.total_suggestions, self.high_impact, self.medium_impact, self.low_impact
        )
    }
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
    fn test_analyze_minimal_config() {
        let content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim
    neovim
    firefox
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let optimizer = ConfigOptimizer::new(&path).unwrap();
        let suggestions = optimizer.analyze();

        // Should detect multiple editors
        assert!(suggestions.iter().any(|s| {
            s.category == Category::UnusedPackages && s.description.contains("Multiple editors")
        }));
    }

    #[test]
    fn test_analyze_config_with_optimizations() {
        let content = r#"{ config, pkgs, ... }: {
  nix.settings.auto-optimise-store = true;
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nix.settings.trusted-users = [ "root" "@wheel" ];

  environment.systemPackages = with pkgs; [
    firefox
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let optimizer = ConfigOptimizer::new(&path).unwrap();
        let suggestions = optimizer.analyze();

        // Should not suggest build optimizations since they're already enabled
        // Note: The test may still get other categories of suggestions
        assert!(
            !suggestions
                .iter()
                .any(|s| { s.category == Category::BuildOptimization }),
            "Build optimization was suggested despite settings being enabled"
        );
    }

    #[test]
    fn test_generate_report() {
        let content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim
    neovim
    emacs
    firefox
    chromium
    google-chrome
  ];
}
"#;
        let (path, _dir) = create_test_config(content);
        let optimizer = ConfigOptimizer::new(&path).unwrap();
        let report = optimizer.generate_report();

        assert!(report.total_suggestions > 0);
        assert!(!report.summary().is_empty());
    }
}
