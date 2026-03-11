//! AI Natural Language Interpreter
//! 
//! Converts natural language requests into package actions.
//! Uses a combination of pattern matching and optional LLM API.

use crate::package_resolver::PackageResolver;
use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Action type to perform
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Install,
    Remove,
    Search,
    Unknown,
}

/// Parsed AI request result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAction {
    pub action: ActionType,
    pub packages: Vec<String>,
    pub confidence: f32,
    pub original_request: String,
}

/// AI Interpreter that converts natural language to actions
pub struct AiInterpreter {
    resolver: PackageResolver,
    api_key: Option<String>,
    api_url: Option<String>,
}

impl AiInterpreter {
    pub fn new(resolver: PackageResolver) -> Self {
        let api_key = std::env::var("SLOP_AI_API_KEY").ok();
        let api_url = std::env::var("SLOP_AI_API_URL").ok();

        AiInterpreter {
            resolver,
            api_key,
            api_url,
        }
    }

    /// Parse a natural language request into an action
    pub fn interpret(&self, request: &str) -> Result<AiAction> {
        let request_lower = request.to_lowercase();

        // Try pattern matching first (fast, offline)
        if let Some(action) = self.pattern_match(&request_lower) {
            return Ok(action);
        }

        // Fall back to LLM if configured
        if self.api_key.is_some() {
            if let Ok(action) = self.llm_interpret(request) {
                return Ok(action);
            }
        }

        // Default: treat as search
        Ok(AiAction {
            action: ActionType::Search,
            packages: vec![request.to_string()],
            confidence: 0.5,
            original_request: request.to_string(),
        })
    }

    /// Pattern-based interpretation (offline, fast)
    fn pattern_match(&self, request: &str) -> Option<AiAction> {
        // Install patterns
        let install_patterns = [
            r"(?:install|add|get|download|setup|configure)\s+(?:a|an|the)?\s*(.+)",
            r"(?:i\s+want|i\s+need|i\s+would\s+like)\s+(?:a|an|the)?\s*(.+)",
            r"(?:give\s+me|show\s+me)\s+(?:a|an|the)?\s*(.+)",
            r"(.+)\s+(?:installer|setup|please)",
        ];

        // Remove patterns
        let remove_patterns = [
            r"(?:remove|delete|uninstall|drop|get\s+rid\s+of)\s+(?:a|an|the)?\s*(.+)",
            r"(?:i\s+don't\s+want|i\s+hate|i\s+dislike)\s+(?:a|an|the)?\s*(.+)",
        ];

        // Check install patterns
        for pattern in install_patterns {
            if let Some(caps) = Regex::new(pattern).ok()?.captures(request) {
                if let Some(matched) = caps.get(1) {
                    let package_query = matched.as_str().trim();
                    let packages = self.resolve_packages(package_query);
                    if !packages.is_empty() {
                        return Some(AiAction {
                            action: ActionType::Install,
                            packages,
                            confidence: 0.8,
                            original_request: request.to_string(),
                        });
                    }
                }
            }
        }

        // Check remove patterns
        for pattern in remove_patterns {
            if let Some(caps) = Regex::new(pattern).ok()?.captures(request) {
                if let Some(matched) = caps.get(1) {
                    let package_query = matched.as_str().trim();
                    let packages = self.resolve_packages(package_query);
                    if !packages.is_empty() {
                        return Some(AiAction {
                            action: ActionType::Remove,
                            packages,
                            confidence: 0.8,
                            original_request: request.to_string(),
                        });
                    }
                }
            }
        }

        // Check for "browser" or "editor" type requests
        let category_keywords = [
            ("browser", "firefox"),
            ("editor", "neovim"),
            ("terminal", "alacritty"),
            ("shell", "zsh"),
            ("git", "git"),
            ("music", "spotify"),
            ("video", "vlc"),
            ("chat", "discord"),
            ("docker", "docker"),
        ];

        for (keyword, package) in category_keywords {
            if request.contains(keyword) {
                return Some(AiAction {
                    action: ActionType::Install,
                    packages: vec![package.to_string()],
                    confidence: 0.7,
                    original_request: request.to_string(),
                });
            }
        }

        None
    }

    /// Resolve a category/description to actual package names
    fn resolve_packages(&self, query: &str) -> Vec<String> {
        // Check if it's a direct package name
        if let Some(resolved) = self.resolver.resolve(query) {
            return vec![resolved.to_string()];
        }

        // Search for matching packages
        let results = self.resolver.search(query);
        if !results.is_empty() {
            return results.into_iter().map(|r| r.attr_name).take(3).collect();
        }

        // Return the query as-is if nothing else works
        vec![query.to_string()]
    }

    /// Use LLM API for interpretation (if configured)
    fn llm_interpret(&self, request: &str) -> Result<AiAction> {
        let api_key = self.api_key.as_ref().context("No API key configured")?;
        let api_url = self.api_url.as_ref().unwrap_or(&"https://api.openai.com/v1/chat/completions".to_string());

        let prompt = format!(
            r#"You are a NixOS package management assistant. Convert this request into a JSON action:

Request: "{}"

Respond with JSON in this format:
{{
    "action": "install" | "remove" | "search",
    "packages": ["package1", "package2"],
    "confidence": 0.0-1.0
}}

Common NixOS packages:
- Browsers: firefox, chromium, google-chrome, brave
- Editors: neovim, vim, emacs, vscode
- Terminals: alacritty, kitty, wezterm, foot
- Shells: zsh, fish, bash, nushell
- Tools: git, htop, tree, ripgrep, fzf, bat

If unsure, use "search" action."#,
            request
        );

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": "You are a helpful NixOS package assistant. Always respond with valid JSON."},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.3,
                "max_tokens": 100
            }))
            .send()
            .context("Failed to send request to LLM API")?;

        if !response.status().is_success() {
            anyhow::bail!("LLM API returned error: {}", response.status());
        }

        #[derive(Deserialize)]
        struct LlmResponse {
            choices: Vec<LlmChoice>,
        }

        #[derive(Deserialize)]
        struct LlmChoice {
            message: LlmMessage,
        }

        #[derive(Deserialize)]
        struct LlmMessage {
            content: String,
        }

        let llm_response: LlmResponse = response.json()
            .context("Failed to parse LLM response")?;

        let content = llm_response
            .choices
            .first()
            .map(|c| c.message.content.as_str())
            .context("Empty LLM response")?;

        // Extract JSON from response (might be wrapped in markdown)
        let json_str = content
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: serde_json::Value = serde_json::from_str(json_str)
            .context("Failed to parse LLM JSON")?;

        let action_str = parsed["action"].as_str().unwrap_or("search");
        let packages = parsed["packages"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let confidence = parsed["confidence"].as_f64().unwrap_or(0.5) as f32;

        let action = match action_str {
            "install" => ActionType::Install,
            "remove" => ActionType::Remove,
            _ => ActionType::Search,
        };

        Ok(AiAction {
            action,
            packages,
            confidence,
            original_request: request.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_interpreter() -> AiInterpreter {
        AiInterpreter::new(PackageResolver::new())
    }

    #[test]
    fn test_install_browser() {
        let interp = create_interpreter();
        let action = interp.interpret("install a browser").unwrap();
        assert_eq!(action.action, ActionType::Install);
        assert!(!action.packages.is_empty());
    }

    #[test]
    fn test_install_neovim() {
        let interp = create_interpreter();
        let action = interp.interpret("i need a terminal editor").unwrap();
        assert_eq!(action.action, ActionType::Install);
        assert!(action.packages.iter().any(|p| p.contains("nvim") || p.contains("vim")));
    }

    #[test]
    fn test_remove_package() {
        let interp = create_interpreter();
        let action = interp.interpret("remove firefox").unwrap();
        assert_eq!(action.action, ActionType::Remove);
    }

    #[test]
    fn test_direct_package_name() {
        let interp = create_interpreter();
        let action = interp.interpret("install firefox").unwrap();
        assert_eq!(action.action, ActionType::Install);
        assert!(action.packages.contains(&"firefox".to_string()));
    }
}
