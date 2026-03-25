//! AI Conflict Detection
//!
//! Detects package conflicts and compatibility issues before installation.

use crate::nix_config::NixConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// Conflict severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Type of conflict
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    DuplicateFunctionality,
    ResourceConflict,
    VersionIncompatibility,
    DependencyConflict,
    ConfigurationConflict,
    PerformanceImpact,
}

/// A detected conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub title: String,
    pub description: String,
    pub packages_involved: Vec<String>,
    pub suggestion: String,
    pub auto_fixable: bool,
}

/// Conflict detector
pub struct ConflictDetector {
    installed_packages: HashSet<String>,
    conflict_rules: Vec<ConflictRule>,
}

/// Rule for detecting conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    pub id: String,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub packages: Vec<String>,
    pub message: String,
    pub suggestion: String,
}

impl ConflictDetector {
    /// Create a new conflict detector
    pub fn new() -> Self {
        let mut detector = ConflictDetector {
            installed_packages: HashSet::new(),
            conflict_rules: Vec::new(),
        };
        detector.init_rules();
        detector
    }

    /// Create from config path
    pub fn from_config<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let mut detector = Self::new();

        if let Ok(config) = NixConfig::load(config_path) {
            detector.installed_packages = config.packages.iter().cloned().collect();
        }

        Ok(detector)
    }

    /// Initialize conflict detection rules
    fn init_rules(&mut self) {
        // Duplicate functionality conflicts
        self.add_rule(ConflictRule {
            id: "editors-multiple".to_string(),
            conflict_type: ConflictType::DuplicateFunctionality,
            severity: ConflictSeverity::Info,
            packages: vec!["vim".to_string(), "neovim".to_string(), "emacs".to_string()],
            message: "Multiple text editors will be installed".to_string(),
            suggestion: "Consider keeping only one primary editor to save space".to_string(),
        });

        self.add_rule(ConflictRule {
            id: "browsers-multiple".to_string(),
            conflict_type: ConflictType::DuplicateFunctionality,
            severity: ConflictSeverity::Info,
            packages: vec![
                "firefox".to_string(),
                "chromium".to_string(),
                "google-chrome".to_string(),
                "brave".to_string(),
            ],
            message: "Multiple web browsers will be installed".to_string(),
            suggestion: "Having multiple browsers is normal, but consider if you need all of them"
                .to_string(),
        });

        self.add_rule(ConflictRule {
            id: "media-players-multiple".to_string(),
            conflict_type: ConflictType::DuplicateFunctionality,
            severity: ConflictSeverity::Info,
            packages: vec![
                "vlc".to_string(),
                "mpv".to_string(),
                "celluloid".to_string(),
            ],
            message: "Multiple media players will be installed".to_string(),
            suggestion: "Consider keeping only one primary media player".to_string(),
        });

        self.add_rule(ConflictRule {
            id: "system-monitors-multiple".to_string(),
            conflict_type: ConflictType::DuplicateFunctionality,
            severity: ConflictSeverity::Info,
            packages: vec!["htop".to_string(), "btop".to_string(), "top".to_string()],
            message: "Multiple system monitors will be installed".to_string(),
            suggestion: "btop is a modern replacement for htop - consider using only one"
                .to_string(),
        });

        // Desktop environment conflicts
        self.add_rule(ConflictRule {
            id: "de-multiple".to_string(),
            conflict_type: ConflictType::ResourceConflict,
            severity: ConflictSeverity::Warning,
            packages: vec!["gnome".to_string(), "plasma".to_string(), "xfce".to_string()],
            message: "Multiple desktop environments will be installed".to_string(),
            suggestion: "Installing multiple DEs can cause conflicts and uses significant disk space (~2GB each)".to_string(),
        });

        // Package replacement conflicts
        self.add_rule(ConflictRule {
            id: "vim-vs-neovim".to_string(),
            conflict_type: ConflictType::ConfigurationConflict,
            severity: ConflictSeverity::Warning,
            packages: vec!["vim".to_string(), "neovim".to_string()],
            message: "Vim and Neovim have different configuration files".to_string(),
            suggestion: "Neovim uses ~/.config/nvim/ instead of ~/.vimrc".to_string(),
        });

        // Docker vs Podman
        self.add_rule(ConflictRule {
            id: "docker-vs-podman".to_string(),
            conflict_type: ConflictType::ResourceConflict,
            severity: ConflictSeverity::Info,
            packages: vec!["docker".to_string(), "podman".to_string()],
            message: "Both Docker and Podman will be installed".to_string(),
            suggestion: "Podman is a daemonless alternative to Docker. You may only need one."
                .to_string(),
        });

        // Python version conflicts
        self.add_rule(ConflictRule {
            id: "python-versions".to_string(),
            conflict_type: ConflictType::VersionIncompatibility,
            severity: ConflictSeverity::Warning,
            packages: vec!["python27".to_string(), "python3".to_string()],
            message: "Multiple Python versions will be installed".to_string(),
            suggestion:
                "Python 2 is EOL. Consider using only Python 3 unless you have legacy needs."
                    .to_string(),
        });

        // Node.js version conflicts
        self.add_rule(ConflictRule {
            id: "nodejs-versions".to_string(),
            conflict_type: ConflictType::VersionIncompatibility,
            severity: ConflictSeverity::Warning,
            packages: vec![
                "nodejs".to_string(),
                "nodejs-14".to_string(),
                "nodejs-16".to_string(),
                "nodejs-18".to_string(),
            ],
            message: "Multiple Node.js versions detected".to_string(),
            suggestion: "Consider using nodeenv or nvm for managing multiple Node.js versions."
                .to_string(),
        });

        // Performance impact warnings
        self.add_rule(ConflictRule {
            id: "heavy-packages".to_string(),
            conflict_type: ConflictType::PerformanceImpact,
            severity: ConflictSeverity::Info,
            packages: vec![
                "libreoffice".to_string(),
                "android-studio".to_string(),
                "intellij-idea".to_string(),
            ],
            message: "Large packages will be installed".to_string(),
            suggestion: "These packages require significant disk space (1GB+) and memory."
                .to_string(),
        });

        // Shell conflicts
        self.add_rule(ConflictRule {
            id: "shells-multiple".to_string(),
            conflict_type: ConflictType::ConfigurationConflict,
            severity: ConflictSeverity::Info,
            packages: vec!["zsh".to_string(), "fish".to_string(), "bash".to_string()],
            message: "Multiple shells will be installed".to_string(),
            suggestion:
                "Having multiple shells is fine, but you'll need to configure each separately."
                    .to_string(),
        });
    }

    /// Add a conflict rule
    fn add_rule(&mut self, rule: ConflictRule) {
        self.conflict_rules.push(rule);
    }

    /// Set installed packages
    pub fn set_installed_packages(&mut self, packages: HashSet<String>) {
        self.installed_packages = packages;
    }

    /// Check for conflicts in a list of packages to install
    pub fn check_conflicts(&self, packages: &[String]) -> Vec<Conflict> {
        let mut conflicts = Vec::new();
        let mut all_packages: HashSet<String> = self.installed_packages.clone();

        // Add packages to be installed
        for pkg in packages {
            all_packages.insert(pkg.clone());
        }

        // Check each rule
        for rule in &self.conflict_rules {
            let matching_packages: Vec<String> = rule
                .packages
                .iter()
                .filter(|p| all_packages.iter().any(|ap| ap.contains(*p)))
                .cloned()
                .collect();

            if matching_packages.len() > 1 {
                conflicts.push(Conflict {
                    id: rule.id.clone(),
                    conflict_type: rule.conflict_type.clone(),
                    severity: rule.severity.clone(),
                    title: rule.message.clone(),
                    description: format!(
                        "The following packages may conflict: {}",
                        matching_packages.join(", ")
                    ),
                    packages_involved: matching_packages,
                    suggestion: rule.suggestion.clone(),
                    auto_fixable: false,
                });
            }
        }

        // Check for packages being installed that are already present
        for pkg in packages {
            if self.installed_packages.contains(pkg) {
                conflicts.push(Conflict {
                    id: format!("already-installed-{}", pkg),
                    conflict_type: ConflictType::DuplicateFunctionality,
                    severity: ConflictSeverity::Info,
                    title: format!("Package '{}' is already installed", pkg),
                    description: format!("The package '{}' is already in your configuration.", pkg),
                    packages_involved: vec![pkg.clone()],
                    suggestion: "You can skip this installation or verify it's working correctly."
                        .to_string(),
                    auto_fixable: true,
                });
            }
        }

        // Sort by severity
        conflicts.sort_by(|a, b| {
            let severity_order = |s: &ConflictSeverity| match s {
                ConflictSeverity::Critical => 0,
                ConflictSeverity::Error => 1,
                ConflictSeverity::Warning => 2,
                ConflictSeverity::Info => 3,
            };
            severity_order(&a.severity).cmp(&severity_order(&b.severity))
        });

        conflicts
    }

    /// Check for dependency conflicts
    pub fn check_dependencies(&self, packages: &[String]) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // Common dependency issues
        let dependency_rules = [
            (
                "rust-analyzer",
                "rustup",
                "Rust analyzer requires rustup to be installed",
            ),
            ("cargo", "rustup", "Cargo is included with rustup"),
            ("clippy", "rustup", "Clippy is included with rustup"),
            ("rustfmt", "rustup", "Rustfmt is included with rustup"),
        ];

        for (pkg, requires, message) in &dependency_rules {
            if packages.iter().any(|p| p.contains(pkg))
                && !packages.iter().any(|p| p.contains(requires))
                && !self.installed_packages.iter().any(|p| p.contains(requires))
            {
                conflicts.push(Conflict {
                    id: format!("missing-dependency-{}", pkg),
                    conflict_type: ConflictType::DependencyConflict,
                    severity: ConflictSeverity::Warning,
                    title: format!("Missing dependency for {}", pkg),
                    description: message.to_string(),
                    packages_involved: vec![pkg.to_string(), requires.to_string()],
                    suggestion: format!("Install {} along with {}", requires, pkg),
                    auto_fixable: true,
                });
            }
        }

        conflicts
    }

    /// Get auto-fix suggestions
    pub fn get_auto_fixes(&self, conflicts: &[Conflict]) -> Vec<(String, String)> {
        let mut fixes = Vec::new();

        for conflict in conflicts {
            if conflict.auto_fixable {
                match conflict.conflict_type {
                    ConflictType::DuplicateFunctionality => {
                        if conflict.title.contains("already installed") {
                            fixes.push((
                                conflict.id.clone(),
                                "Skip this package installation".to_string(),
                            ));
                        }
                    }
                    ConflictType::DependencyConflict => {
                        fixes.push((
                            conflict.id.clone(),
                            format!("Auto-add missing dependency: {}", conflict.suggestion),
                        ));
                    }
                    _ => {}
                }
            }
        }

        fixes
    }

    /// Generate a conflict report
    pub fn generate_report(&self, packages: &[String]) -> ConflictReport {
        let conflicts = self.check_conflicts(packages);
        let dependency_conflicts = self.check_dependencies(packages);

        let mut all_conflicts = conflicts;
        all_conflicts.extend(dependency_conflicts);

        let critical = all_conflicts
            .iter()
            .filter(|c| c.severity == ConflictSeverity::Critical)
            .count();
        let errors = all_conflicts
            .iter()
            .filter(|c| c.severity == ConflictSeverity::Error)
            .count();
        let warnings = all_conflicts
            .iter()
            .filter(|c| c.severity == ConflictSeverity::Warning)
            .count();
        let info = all_conflicts
            .iter()
            .filter(|c| c.severity == ConflictSeverity::Info)
            .count();

        ConflictReport {
            total: all_conflicts.len(),
            critical,
            errors,
            warnings,
            info,
            conflicts: all_conflicts,
            can_proceed: critical == 0 && errors == 0,
        }
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Conflict report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictReport {
    pub total: usize,
    pub critical: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub conflicts: Vec<Conflict>,
    pub can_proceed: bool,
}

impl ConflictReport {
    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "Found {} conflict(s): {} critical, {} errors, {} warnings, {} info",
            self.total, self.critical, self.errors, self.warnings, self.info
        )
    }

    /// Check if installation can proceed
    pub fn is_safe(&self) -> bool {
        self.can_proceed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_detector_creation() {
        let detector = ConflictDetector::new();
        assert!(!detector.conflict_rules.is_empty());
    }

    #[test]
    fn test_detect_multiple_editors() {
        let detector = ConflictDetector::new();
        let conflicts = detector.check_conflicts(&["vim".to_string(), "neovim".to_string()]);

        assert!(!conflicts.is_empty());
        assert!(conflicts.iter().any(|c| c.id == "editors-multiple"));
    }

    #[test]
    fn test_detect_multiple_browsers() {
        let detector = ConflictDetector::new();
        let conflicts = detector.check_conflicts(&[
            "firefox".to_string(),
            "chromium".to_string(),
            "google-chrome".to_string(),
        ]);

        assert!(conflicts.iter().any(|c| c.id == "browsers-multiple"));
    }

    #[test]
    fn test_detect_already_installed() {
        let mut detector = ConflictDetector::new();
        detector.installed_packages.insert("firefox".to_string());

        let conflicts = detector.check_conflicts(&["firefox".to_string()]);

        assert!(conflicts
            .iter()
            .any(|c| c.id.starts_with("already-installed")));
    }

    #[test]
    fn test_generate_report() {
        let detector = ConflictDetector::new();
        let packages = vec![
            "vim".to_string(),
            "neovim".to_string(),
            "firefox".to_string(),
        ];

        let report = detector.generate_report(&packages);

        assert!(report.total > 0);
        assert!(!report.summary().is_empty());
    }

    #[test]
    fn test_no_conflicts() {
        let detector = ConflictDetector::new();
        let conflicts = detector.check_conflicts(&["firefox".to_string()]);

        // Should have minimal or no conflicts for single package
        assert!(conflicts.len() <= 1);
    }
}
