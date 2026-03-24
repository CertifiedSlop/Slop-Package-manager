//! AI Semantic Search
//!
//! Semantic package search beyond keyword matching.

use crate::package_resolver::{PackageInfo, PackageResolver, SearchResult};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Semantic search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    pub attr_name: String,
    pub package: PackageInfo,
    pub score: f32,
    pub match_type: MatchType,
    pub reasoning: String,
}

/// Type of match
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
    Exact,
    Semantic,
    Category,
    Related,
    Fuzzy,
}

/// Package category for semantic grouping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCategory {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub example_packages: Vec<String>,
}

/// Semantic search engine
pub struct SemanticSearchEngine {
    resolver: PackageResolver,
    categories: Vec<PackageCategory>,
    package_descriptions: HashMap<String, String>,
    keyword_index: HashMap<String, Vec<String>>,
}

impl SemanticSearchEngine {
    /// Create a new semantic search engine
    pub fn new(resolver: PackageResolver) -> Self {
        let mut engine = SemanticSearchEngine {
            resolver,
            categories: Vec::new(),
            package_descriptions: HashMap::new(),
            keyword_index: HashMap::new(),
        };
        engine.init_categories();
        engine.init_keyword_index();
        engine
    }

    /// Initialize package categories
    fn init_categories(&mut self) {
        self.categories = vec![
            PackageCategory {
                name: "Web Browsers".to_string(),
                description: "Web browsing and internet navigation".to_string(),
                keywords: vec![
                    "browser".to_string(),
                    "web".to_string(),
                    "internet".to_string(),
                    "surf".to_string(),
                    "navigate".to_string(),
                ],
                example_packages: vec!["firefox".to_string(), "chromium".to_string()],
            },
            PackageCategory {
                name: "Text Editors".to_string(),
                description: "Text editing and code editing".to_string(),
                keywords: vec![
                    "editor".to_string(),
                    "text".to_string(),
                    "code".to_string(),
                    "write".to_string(),
                    "edit".to_string(),
                    "ide".to_string(),
                ],
                example_packages: vec!["neovim".to_string(), "vscode".to_string(), "emacs".to_string()],
            },
            PackageCategory {
                name: "Terminal Emulators".to_string(),
                description: "Terminal and console applications".to_string(),
                keywords: vec![
                    "terminal".to_string(),
                    "console".to_string(),
                    "shell".to_string(),
                    "command".to_string(),
                    "tty".to_string(),
                ],
                example_packages: vec!["alacritty".to_string(), "kitty".to_string(), "wezterm".to_string()],
            },
            PackageCategory {
                name: "System Monitoring".to_string(),
                description: "System resource monitoring and diagnostics".to_string(),
                keywords: vec![
                    "monitor".to_string(),
                    "system".to_string(),
                    "resource".to_string(),
                    "performance".to_string(),
                    "top".to_string(),
                    "stats".to_string(),
                ],
                example_packages: vec!["htop".to_string(), "btop".to_string(), "glances".to_string()],
            },
            PackageCategory {
                name: "File Management".to_string(),
                description: "File browsing and management tools".to_string(),
                keywords: vec![
                    "file".to_string(),
                    "manager".to_string(),
                    "browser".to_string(),
                    "explore".to_string(),
                    "directory".to_string(),
                    "ls".to_string(),
                ],
                example_packages: vec!["ranger".to_string(), "nnn".to_string(), "lf".to_string(), "eza".to_string()],
            },
            PackageCategory {
                name: "Development Tools".to_string(),
                description: "Programming and development utilities".to_string(),
                keywords: vec![
                    "development".to_string(),
                    "programming".to_string(),
                    "coding".to_string(),
                    "build".to_string(),
                    "compile".to_string(),
                    "debug".to_string(),
                ],
                example_packages: vec!["git".to_string(), "cargo".to_string(), "gcc".to_string()],
            },
            PackageCategory {
                name: "Media Players".to_string(),
                description: "Audio and video playback".to_string(),
                keywords: vec![
                    "media".to_string(),
                    "player".to_string(),
                    "video".to_string(),
                    "audio".to_string(),
                    "music".to_string(),
                    "watch".to_string(),
                    "listen".to_string(),
                ],
                example_packages: vec!["vlc".to_string(), "mpv".to_string(), "spotify".to_string()],
            },
            PackageCategory {
                name: "Image Editing".to_string(),
                description: "Image viewing and editing".to_string(),
                keywords: vec![
                    "image".to_string(),
                    "photo".to_string(),
                    "picture".to_string(),
                    "edit".to_string(),
                    "graphics".to_string(),
                    "draw".to_string(),
                ],
                example_packages: vec!["gimp".to_string(), "inkscape".to_string(), "nomacs".to_string()],
            },
            PackageCategory {
                name: "Video Editing".to_string(),
                description: "Video editing and production".to_string(),
                keywords: vec![
                    "video".to_string(),
                    "edit".to_string(),
                    "movie".to_string(),
                    "film".to_string(),
                    "render".to_string(),
                    "production".to_string(),
                ],
                example_packages: vec!["kdenlive".to_string(), "obs-studio".to_string(), "shotcut".to_string()],
            },
            PackageCategory {
                name: "Communication".to_string(),
                description: "Chat and communication tools".to_string(),
                keywords: vec![
                    "chat".to_string(),
                    "message".to_string(),
                    "communicate".to_string(),
                    "talk".to_string(),
                    "social".to_string(),
                ],
                example_packages: vec!["discord".to_string(), "telegram-desktop".to_string(), "signal-desktop".to_string()],
            },
            PackageCategory {
                name: "Networking Tools".to_string(),
                description: "Network utilities and diagnostics".to_string(),
                keywords: vec![
                    "network".to_string(),
                    "internet".to_string(),
                    "download".to_string(),
                    "upload".to_string(),
                    "scan".to_string(),
                    "connect".to_string(),
                ],
                example_packages: vec!["curl".to_string(), "wget".to_string(), "nmap".to_string()],
            },
            PackageCategory {
                name: "Security Tools".to_string(),
                description: "Security and privacy utilities".to_string(),
                keywords: vec![
                    "security".to_string(),
                    "privacy".to_string(),
                    "encrypt".to_string(),
                    "protect".to_string(),
                    "password".to_string(),
                ],
                example_packages: vec!["gnupg".to_string(), "keepassxc".to_string(), "veracrypt".to_string()],
            },
            PackageCategory {
                name: "Gaming".to_string(),
                description: "Gaming platforms and tools".to_string(),
                keywords: vec![
                    "game".to_string(),
                    "gaming".to_string(),
                    "play".to_string(),
                    "steam".to_string(),
                    "xbox".to_string(),
                ],
                example_packages: vec!["steam".to_string(), "lutris".to_string(), "gamescope".to_string()],
            },
            PackageCategory {
                name: "Office/Productivity".to_string(),
                description: "Office suites and productivity tools".to_string(),
                keywords: vec![
                    "office".to_string(),
                    "productivity".to_string(),
                    "document".to_string(),
                    "spreadsheet".to_string(),
                    "presentation".to_string(),
                    "word".to_string(),
                ],
                example_packages: vec!["libreoffice".to_string(), "onlyoffice".to_string()],
            },
        ];
    }

    /// Initialize keyword index for fast lookups
    fn init_keyword_index(&mut self) {
        // Map common terms to package categories
        let keyword_mappings = [
            ("browser", "Web Browsers"),
            ("web", "Web Browsers"),
            ("internet", "Web Browsers"),
            ("editor", "Text Editors"),
            ("text", "Text Editors"),
            ("code", "Text Editors"),
            ("terminal", "Terminal Emulators"),
            ("console", "Terminal Emulators"),
            ("monitor", "System Monitoring"),
            ("system", "System Monitoring"),
            ("file", "File Management"),
            ("manager", "File Management"),
            ("development", "Development Tools"),
            ("programming", "Development Tools"),
            ("media", "Media Players"),
            ("video", "Media Players"),
            ("music", "Media Players"),
            ("image", "Image Editing"),
            ("photo", "Image Editing"),
            ("chat", "Communication"),
            ("network", "Networking Tools"),
            ("security", "Security Tools"),
            ("game", "Gaming"),
            ("office", "Office/Productivity"),
        ];

        for (keyword, category) in keyword_mappings {
            self.keyword_index
                .entry(keyword.to_string())
                .or_insert_with(Vec::new)
                .push(category.to_string());
        }
    }

    /// Perform semantic search
    pub fn search(&self, query: &str) -> Vec<SemanticSearchResult> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // First, try to find matching categories
        let matching_categories = self.find_matching_categories(&query_lower);

        // Get traditional search results
        let traditional_results = self.resolver.search(&query_lower);

        // Add traditional results with semantic enhancement
        for result in traditional_results {
            let (match_type, reasoning) = self.determine_match_type(&result, &matching_categories);
            
            results.push(SemanticSearchResult {
                attr_name: result.attr_name,
                package: result.package,
                score: result.score,
                match_type,
                reasoning,
            });
        }

        // Add related packages from matching categories
        for category in &matching_categories {
            for example_pkg in &category.example_packages {
                if !results.iter().any(|r| r.attr_name == *example_pkg) {
                    if let Some(resolved) = self.resolver.resolve(example_pkg) {
                        results.push(SemanticSearchResult {
                            attr_name: resolved.to_string(),
                            package: PackageInfo {
                                name: example_pkg.clone(),
                                pname: Some(resolved.to_string()),
                                version: None,
                                description: Some(format!("From category: {}", category.name)),
                                license: None,
                                homepage: None,
                            },
                            score: 0.6,
                            match_type: MatchType::Category,
                            reasoning: format!("Related to: {}", category.name),
                        });
                    }
                }
            }
        }

        // Sort by score and match type priority
        results.sort_by(|a, b| {
            let type_priority = |t: &MatchType| match t {
                MatchType::Exact => 0,
                MatchType::Semantic => 1,
                MatchType::Category => 2,
                MatchType::Related => 3,
                MatchType::Fuzzy => 4,
            };
            
            type_priority(&a.match_type)
                .cmp(&type_priority(&b.match_type))
                .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
        });

        results
    }

    /// Find categories matching the query
    fn find_matching_categories(&self, query: &str) -> Vec<&PackageCategory> {
        let mut matching = Vec::new();

        for category in &self.categories {
            let mut score = 0;

            // Check if query matches category name
            if category.name.to_lowercase().contains(query) {
                score += 3;
            }

            // Check if query matches description
            if category.description.to_lowercase().contains(query) {
                score += 2;
            }

            // Check keywords
            for keyword in &category.keywords {
                if keyword.contains(query) || query.contains(keyword) {
                    score += 1;
                }
            }

            if score > 0 {
                matching.push(category);
            }
        }

        matching.sort_by(|a, b| {
            let a_score = self.calculate_category_score(a, query);
            let b_score = self.calculate_category_score(b, query);
            b_score.cmp(&a_score)
        });

        matching
    }

    /// Calculate category match score
    fn calculate_category_score(&self, category: &PackageCategory, query: &str) -> usize {
        let mut score = 0;

        if category.name.to_lowercase().contains(query) {
            score += 10;
        }

        for keyword in &category.keywords {
            if keyword == query {
                score += 5;
            } else if keyword.contains(query) || query.contains(keyword) {
                score += 2;
            }
        }

        score
    }

    /// Determine the type of match
    fn determine_match_type(
        &self,
        result: &SearchResult,
        categories: &[&PackageCategory],
    ) -> (MatchType, String) {
        let attr_lower = result.attr_name.to_lowercase();
        let name_lower = result.package.name.to_lowercase();

        // Check for exact match
        if attr_lower == name_lower {
            return (
                MatchType::Exact,
                "Exact package name match".to_string(),
            );
        }

        // Check for category match
        for category in categories {
            if category.example_packages.iter().any(|p| p == &result.attr_name) {
                return (
                    MatchType::Category,
                    format!("Found in category: {}", category.name),
                );
            }
        }

        // Check for semantic match based on description
        if let Some(desc) = &result.package.description {
            if desc.to_lowercase().contains(&name_lower) {
                return (
                    MatchType::Semantic,
                    "Matched by description".to_string(),
                );
            }
        }

        // Default to fuzzy match
        (
            MatchType::Fuzzy,
            "Fuzzy/text match".to_string(),
        )
    }

    /// Search by use case
    pub fn search_by_use_case(&self, use_case: &str) -> Vec<SemanticSearchResult> {
        let use_case_lower = use_case.to_lowercase();
        
        // Map use cases to package recommendations
        let use_case_packages: HashMap<&str, Vec<&str>> = [
            ("web browsing", vec!["firefox", "chromium"]),
            ("text editing", vec!["neovim", "vscode"]),
            ("video editing", vec!["kdenlive", "obs-studio", "shotcut"]),
            ("image editing", vec!["gimp", "inkscape"]),
            ("music listening", vec!["spotify", "mpv"]),
            ("video watching", vec!["vlc", "mpv"]),
            ("chat messaging", vec!["discord", "telegram-desktop"]),
            ("programming rust", vec!["rustup", "cargo", "rust-analyzer"]),
            ("programming python", vec!["python3", "pip", "uv"]),
            ("programming web", vec!["nodejs", "vscode", "chromium"]),
            ("system monitoring", vec!["htop", "btop", "glances"]),
            ("file browsing", vec!["ranger", "nnn", "lf"]),
            ("gaming", vec!["steam", "lutris", "gamescope"]),
            ("office work", vec!["libreoffice", "onlyoffice"]),
            ("security privacy", vec!["gnupg", "keepassxc", "veracrypt"]),
        ].iter().cloned().collect();

        let mut results = Vec::new();

        for (case, packages) in &use_case_packages {
            if use_case_lower.contains(case) || case.contains(&use_case_lower) {
                for pkg in packages {
                    if let Some(resolved) = self.resolver.resolve(pkg) {
                        results.push(SemanticSearchResult {
                            attr_name: resolved.to_string(),
                            package: PackageInfo {
                                name: pkg.to_string(),
                                pname: Some(resolved.to_string()),
                                version: None,
                                description: Some(format!("For: {}", case)),
                                license: None,
                                homepage: None,
                            },
                            score: 0.9,
                            match_type: MatchType::Semantic,
                            reasoning: format!("Recommended for: {}", case),
                        });
                    }
                }
            }
        }

        results
    }

    /// Get all available categories
    pub fn get_categories(&self) -> Vec<&PackageCategory> {
        self.categories.iter().collect()
    }

    /// Search within a specific category
    pub fn search_in_category(&self, query: &str, category_name: &str) -> Vec<SemanticSearchResult> {
        let category = self.categories.iter().find(|c| c.name == category_name);
        
        let Some(category) = category else {
            return Vec::new();
        };

        let mut results = Vec::new();

        for example_pkg in &category.example_packages {
            if let Some(resolved) = self.resolver.resolve(example_pkg) {
                let score = if example_pkg.to_lowercase().contains(&query.to_lowercase()) {
                    0.9
                } else {
                    0.7
                };

                results.push(SemanticSearchResult {
                    attr_name: resolved.to_string(),
                    package: PackageInfo {
                        name: example_pkg.clone(),
                        pname: Some(resolved.to_string()),
                        version: None,
                        description: Some(format!("Category: {}", category.name)),
                        license: None,
                        homepage: None,
                    },
                    score,
                    match_type: MatchType::Category,
                    reasoning: format!("In category: {}", category.name),
                });
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let resolver = PackageResolver::new();
        let engine = SemanticSearchEngine::new(resolver);
        
        assert!(!engine.categories.is_empty());
        assert!(!engine.keyword_index.is_empty());
    }

    #[test]
    fn test_semantic_search_browser() {
        let resolver = PackageResolver::new();
        let engine = SemanticSearchEngine::new(resolver);
        
        let results = engine.search("browser");
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.match_type == MatchType::Category));
    }

    #[test]
    fn test_search_by_use_case() {
        let resolver = PackageResolver::new();
        let engine = SemanticSearchEngine::new(resolver);
        
        let results = engine.search_by_use_case("video editing");
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.attr_name == "kdenlive" || r.attr_name == "obs-studio"));
    }

    #[test]
    fn test_get_categories() {
        let resolver = PackageResolver::new();
        let engine = SemanticSearchEngine::new(resolver);
        
        let categories = engine.get_categories();
        
        assert!(categories.len() > 10);
    }
}
