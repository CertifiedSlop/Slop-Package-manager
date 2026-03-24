//! AI Package Bundles
//!
//! Pre-defined package bundles for common use cases.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Package bundle definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageBundle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub packages: Vec<String>,
    pub optional_packages: Vec<String>,
    pub conflicts: Vec<String>,
    pub tags: Vec<String>,
}

/// Bundle manager
pub struct BundleManager {
    bundles: HashMap<String, PackageBundle>,
}

impl BundleManager {
    /// Create a new bundle manager with default bundles
    pub fn new() -> Self {
        let mut manager = BundleManager {
            bundles: HashMap::new(),
        };
        manager.init_bundles();
        manager
    }

    /// Initialize default bundles
    fn init_bundles(&mut self) {
        // Development bundles
        self.add_bundle(PackageBundle {
            id: "dev-rust".to_string(),
            name: "Rust Development".to_string(),
            description: "Complete Rust development environment".to_string(),
            category: "development".to_string(),
            packages: vec![
                "rustup".to_string(),
                "cargo".to_string(),
                "rustfmt".to_string(),
                "clippy".to_string(),
            ],
            optional_packages: vec![
                "rust-analyzer".to_string(),
                "cargo-edit".to_string(),
                "cargo-watch".to_string(),
                "bacon".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "rust".to_string(),
                "development".to_string(),
                "programming".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "dev-python".to_string(),
            name: "Python Development".to_string(),
            description: "Python development environment with tools".to_string(),
            category: "development".to_string(),
            packages: vec![
                "python3".to_string(),
                "pip".to_string(),
                "venv".to_string(),
                "uv".to_string(),
            ],
            optional_packages: vec![
                "pyright".to_string(),
                "black".to_string(),
                "ruff".to_string(),
                "pytest".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "python".to_string(),
                "development".to_string(),
                "programming".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "dev-web".to_string(),
            name: "Web Development".to_string(),
            description: "Web development with Node.js and tools".to_string(),
            category: "development".to_string(),
            packages: vec!["nodejs".to_string(), "npm".to_string(), "yarn".to_string()],
            optional_packages: vec![
                "nodePackages.typescript".to_string(),
                "nodePackages.typescript-language-server".to_string(),
                "nodePackages.prettier".to_string(),
                "nodePackages.eslint".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "javascript".to_string(),
                "typescript".to_string(),
                "web".to_string(),
                "development".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "dev-go".to_string(),
            name: "Go Development".to_string(),
            description: "Go programming language environment".to_string(),
            category: "development".to_string(),
            packages: vec!["go".to_string()],
            optional_packages: vec![
                "go-tools".to_string(),
                "gopls".to_string(),
                "delve".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "go".to_string(),
                "development".to_string(),
                "programming".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "dev-cpp".to_string(),
            name: "C/C++ Development".to_string(),
            description: "C and C++ development tools".to_string(),
            category: "development".to_string(),
            packages: vec![
                "gcc".to_string(),
                "gdb".to_string(),
                "make".to_string(),
                "cmake".to_string(),
            ],
            optional_packages: vec![
                "clang".to_string(),
                "clang-tools".to_string(),
                "lldb".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "c".to_string(),
                "cpp".to_string(),
                "development".to_string(),
            ],
        });

        // Desktop bundles
        self.add_bundle(PackageBundle {
            id: "desktop-essentials".to_string(),
            name: "Desktop Essentials".to_string(),
            description: "Essential desktop applications".to_string(),
            category: "desktop".to_string(),
            packages: vec![
                "firefox".to_string(),
                "neovim".to_string(),
                "alacritty".to_string(),
                "git".to_string(),
            ],
            optional_packages: vec![
                "thunderbird".to_string(),
                "libreoffice".to_string(),
                "file-roller".to_string(),
            ],
            conflicts: vec![],
            tags: vec!["desktop".to_string(), "essential".to_string()],
        });

        self.add_bundle(PackageBundle {
            id: "desktop-media".to_string(),
            name: "Media Bundle".to_string(),
            description: "Media playback and editing tools".to_string(),
            category: "media".to_string(),
            packages: vec!["vlc".to_string(), "mpv".to_string(), "ffmpeg".to_string()],
            optional_packages: vec![
                "obs-studio".to_string(),
                "audacity".to_string(),
                "kdenlive".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "media".to_string(),
                "video".to_string(),
                "audio".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "desktop-gaming".to_string(),
            name: "Gaming Bundle".to_string(),
            description: "Gaming tools and platforms".to_string(),
            category: "gaming".to_string(),
            packages: vec!["steam".to_string(), "gamescope".to_string()],
            optional_packages: vec![
                "lutris".to_string(),
                "heroic".to_string(),
                "mangoapp".to_string(),
                "gamemode".to_string(),
            ],
            conflicts: vec![],
            tags: vec!["gaming".to_string(), "steam".to_string()],
        });

        // Utility bundles
        self.add_bundle(PackageBundle {
            id: "utils-cli".to_string(),
            name: "CLI Power Tools".to_string(),
            description: "Essential command-line utilities".to_string(),
            category: "utilities".to_string(),
            packages: vec![
                "ripgrep".to_string(),
                "fd".to_string(),
                "fzf".to_string(),
                "bat".to_string(),
                "eza".to_string(),
            ],
            optional_packages: vec![
                "zoxide".to_string(),
                "starship".to_string(),
                "jq".to_string(),
                "httpie".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "cli".to_string(),
                "utilities".to_string(),
                "terminal".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "utils-system".to_string(),
            name: "System Monitoring".to_string(),
            description: "System monitoring and diagnostics".to_string(),
            category: "utilities".to_string(),
            packages: vec!["htop".to_string(), "btop".to_string(), "ncdu".to_string()],
            optional_packages: vec![
                "powertop".to_string(),
                "iotop".to_string(),
                "lm_sensors".to_string(),
            ],
            conflicts: vec![],
            tags: vec![
                "system".to_string(),
                "monitoring".to_string(),
                "utilities".to_string(),
            ],
        });

        self.add_bundle(PackageBundle {
            id: "utils-network".to_string(),
            name: "Network Tools".to_string(),
            description: "Network utilities and diagnostics".to_string(),
            category: "utilities".to_string(),
            packages: vec![
                "curl".to_string(),
                "wget".to_string(),
                "dnsutils".to_string(),
            ],
            optional_packages: vec![
                "nmap".to_string(),
                "wireshark".to_string(),
                "netcat".to_string(),
                "httpie".to_string(),
            ],
            conflicts: vec![],
            tags: vec!["network".to_string(), "utilities".to_string()],
        });

        // Security bundles
        self.add_bundle(PackageBundle {
            id: "security-basics".to_string(),
            name: "Security Basics".to_string(),
            description: "Essential security tools".to_string(),
            category: "security".to_string(),
            packages: vec!["gnupg".to_string(), "password-store".to_string()],
            optional_packages: vec![
                "veracrypt".to_string(),
                "keepassxc".to_string(),
                "rkhunter".to_string(),
            ],
            conflicts: vec![],
            tags: vec!["security".to_string(), "encryption".to_string()],
        });

        // Container bundles
        self.add_bundle(PackageBundle {
            id: "containers-docker".to_string(),
            name: "Docker & Containers".to_string(),
            description: "Docker and container management".to_string(),
            category: "development".to_string(),
            packages: vec!["docker".to_string(), "docker-compose".to_string()],
            optional_packages: vec![
                "podman".to_string(),
                "lazydocker".to_string(),
                "containerd".to_string(),
            ],
            conflicts: vec!["podman".to_string()],
            tags: vec![
                "containers".to_string(),
                "docker".to_string(),
                "development".to_string(),
            ],
        });
    }

    /// Add a bundle
    fn add_bundle(&mut self, bundle: PackageBundle) {
        self.bundles.insert(bundle.id.clone(), bundle);
    }

    /// Get a bundle by ID
    pub fn get_bundle(&self, id: &str) -> Option<&PackageBundle> {
        self.bundles.get(id)
    }

    /// Get all bundles in a category
    pub fn get_bundles_by_category(&self, category: &str) -> Vec<&PackageBundle> {
        self.bundles
            .values()
            .filter(|b| b.category == category)
            .collect()
    }

    /// Search bundles by tags
    pub fn search_bundles(&self, query: &str) -> Vec<&PackageBundle> {
        let query_lower = query.to_lowercase();
        self.bundles
            .values()
            .filter(|b| {
                b.id.contains(&query_lower)
                    || b.name.to_lowercase().contains(&query_lower)
                    || b.description.to_lowercase().contains(&query_lower)
                    || b.tags.iter().any(|t| t.contains(&query_lower))
            })
            .collect()
    }

    /// Get bundle suggestions based on keywords
    pub fn suggest_bundles(&self, keywords: &[String]) -> Vec<&PackageBundle> {
        let mut suggestions = Vec::new();

        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            // Match keywords to bundles
            if keyword_lower.contains("rust") {
                if let Some(bundle) = self.get_bundle("dev-rust") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("python") {
                if let Some(bundle) = self.get_bundle("dev-python") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("web")
                || keyword_lower.contains("javascript")
                || keyword_lower.contains("node")
            {
                if let Some(bundle) = self.get_bundle("dev-web") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("go") || keyword_lower.contains("golang") {
                if let Some(bundle) = self.get_bundle("dev-go") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("gaming")
                || keyword_lower.contains("game")
                || keyword_lower.contains("steam")
            {
                if let Some(bundle) = self.get_bundle("desktop-gaming") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("media")
                || keyword_lower.contains("video")
                || keyword_lower.contains("audio")
            {
                if let Some(bundle) = self.get_bundle("desktop-media") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("cli") || keyword_lower.contains("terminal") {
                if let Some(bundle) = self.get_bundle("utils-cli") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("security") {
                if let Some(bundle) = self.get_bundle("security-basics") {
                    suggestions.push(bundle);
                }
            }
            if keyword_lower.contains("docker") || keyword_lower.contains("container") {
                if let Some(bundle) = self.get_bundle("containers-docker") {
                    suggestions.push(bundle);
                }
            }
        }

        // Remove duplicates
        suggestions.sort_by(|a, b| a.id.cmp(&b.id));
        suggestions.dedup();

        suggestions
    }

    /// Get all bundles
    pub fn all_bundles(&self) -> Vec<&PackageBundle> {
        self.bundles.values().collect()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> =
            self.bundles.values().map(|b| b.category.clone()).collect();
        categories.sort();
        categories.dedup();
        categories
    }
}

impl Default for BundleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_manager_creation() {
        let manager = BundleManager::new();
        assert!(manager.all_bundles().len() > 0);
    }

    #[test]
    fn test_get_bundle() {
        let manager = BundleManager::new();
        let bundle = manager.get_bundle("dev-rust");
        assert!(bundle.is_some());
        assert_eq!(bundle.unwrap().name, "Rust Development");
    }

    #[test]
    fn test_search_bundles() {
        let manager = BundleManager::new();
        let results = manager.search_bundles("rust");
        assert!(results.iter().any(|b| b.id == "dev-rust"));
    }

    #[test]
    fn test_suggest_bundles() {
        let manager = BundleManager::new();
        let keywords = vec!["rust".to_string(), "python".to_string()];
        let suggestions = manager.suggest_bundles(&keywords);

        assert!(suggestions.iter().any(|b| b.id == "dev-rust"));
        assert!(suggestions.iter().any(|b| b.id == "dev-python"));
    }

    #[test]
    fn test_get_categories() {
        let manager = BundleManager::new();
        let categories = manager.get_categories();

        assert!(categories.contains(&"development".to_string()));
        assert!(categories.contains(&"utilities".to_string()));
    }
}
