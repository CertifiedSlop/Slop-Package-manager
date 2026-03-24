//! AI Natural Language Interpreter
//!
//! Converts natural language requests into package actions.
//! Supports multiple LLM providers: OpenAI, Ollama, and OpenRouter.

use crate::ai_context::AiContext;
use crate::package_resolver::PackageResolver;
use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Action type to perform
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Install,
    Remove,
    Search,
    Rollback,
    Optimize,
    Suggest,
    Unknown,
}

/// Parsed AI request result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAction {
    pub action: ActionType,
    pub packages: Vec<String>,
    pub confidence: f32,
    pub original_request: String,
    /// Optional rollback target (generation number or time expression)
    pub rollback_target: Option<String>,
    /// Optional optimization flags
    pub optimize_flags: Vec<String>,
    /// Context-aware suggestions
    pub suggestions: Vec<AiSuggestion>,
}

/// AI suggestion for packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub package: String,
    pub reason: String,
    pub category: String,
}

/// LLM Provider configuration
#[derive(Debug, Clone)]
pub enum LlmProvider {
    OpenAI,
    Ollama { url: String },
    OpenRouter,
}

/// AI Interpreter that converts natural language to actions
pub struct AiInterpreter {
    resolver: PackageResolver,
    provider: LlmProvider,
    api_key: Option<String>,
    context: Option<AiContext>,
}

impl AiInterpreter {
    pub fn new(resolver: PackageResolver) -> Self {
        let api_key = std::env::var("SLOP_AI_API_KEY").ok();

        // Determine provider from environment
        let provider = if let Ok(url) = std::env::var("SLOP_OLLAMA_URL") {
            LlmProvider::Ollama { url }
        } else if std::env::var("SLOP_OPENROUTER_KEY").is_ok() {
            LlmProvider::OpenRouter
        } else {
            LlmProvider::OpenAI
        };

        // Try to load context, fall back to None if unavailable
        let context = AiContext::new("/etc/nixos/configuration.nix").ok();

        AiInterpreter {
            resolver,
            provider,
            api_key,
            context,
        }
    }

    /// Create a new interpreter with a custom context
    pub fn with_context(resolver: PackageResolver, context: AiContext) -> Self {
        let api_key = std::env::var("SLOP_AI_API_KEY").ok();

        let provider = if let Ok(url) = std::env::var("SLOP_OLLAMA_URL") {
            LlmProvider::Ollama { url }
        } else if std::env::var("SLOP_OPENROUTER_KEY").is_ok() {
            LlmProvider::OpenRouter
        } else {
            LlmProvider::OpenAI
        };

        AiInterpreter {
            resolver,
            provider,
            api_key,
            context: Some(context),
        }
    }

    /// Parse a natural language request into an action
    pub fn interpret(&self, request: &str) -> Result<AiAction> {
        let request_lower = request.to_lowercase();

        // Check for rollback requests first
        if let Some(action) = self.parse_rollback(&request_lower) {
            return Ok(action);
        }

        // Check for optimization requests
        if let Some(action) = self.parse_optimize(&request_lower) {
            return Ok(action);
        }

        // Check for suggestion requests
        if let Some(action) = self.parse_suggest(&request_lower) {
            return Ok(action);
        }

        // Try pattern matching first (fast, offline)
        if let Some(action) = self.pattern_match(&request_lower) {
            return Ok(action);
        }

        // Fall back to LLM if configured
        match self.llm_interpret(request) {
            Ok(action) => return Ok(action),
            Err(e) => {
                tracing::warn!("LLM interpretation failed: {}", e);
                // Continue to default fallback
            }
        }

        // Default: treat as search
        Ok(AiAction {
            action: ActionType::Search,
            packages: vec![request.to_string()],
            confidence: 0.5,
            original_request: request.to_string(),
            rollback_target: None,
            optimize_flags: Vec::new(),
            suggestions: Vec::new(),
        })
    }

    /// Parse rollback requests
    fn parse_rollback(&self, request: &str) -> Option<AiAction> {
        let rollback_patterns = [
            r"(?:undo|revert|rollback|roll\s*back)\s*(?:my|the)?\s*(?:last|previous|recent)?\s*(?:change|update|modification)?",
            r"(?:go\s+back\s+to)\s+(?:generation\s+)?(\d+)",
            r"(?:undo)\s+(?:my\s+)?(?:last|previous)\s*(?:change|action)",
            r"(?:revert)\s+(?:to\s+)?(?:yesterday|last\s+week|last\s+month)",
        ];

        for pattern in rollback_patterns {
            if let Some(caps) = Regex::new(pattern).ok()?.captures(request) {
                let target = caps.get(1).map(|m| m.as_str().to_string());
                return Some(AiAction {
                    action: ActionType::Rollback,
                    packages: Vec::new(),
                    confidence: 0.9,
                    original_request: request.to_string(),
                    rollback_target: target.or_else(|| Some("last".to_string())),
                    optimize_flags: Vec::new(),
                    suggestions: Vec::new(),
                });
            }
        }
        None
    }

    /// Parse optimization requests
    fn parse_optimize(&self, request: &str) -> Option<AiAction> {
        let optimize_patterns = [
            r"(?:optimize|improve|clean\s*up)\s*(?:my)?\s*(?:config|configuration|system)",
            r"(?:make\s+my)?\s*(?:config|system)\s+(?:faster|smaller|better|cleaner)",
            r"(?:remove\s+unused|clean\s+unused)\s*(?:packages)?",
        ];

        for pattern in optimize_patterns {
            if Regex::new(pattern).ok()?.is_match(request) {
                return Some(AiAction {
                    action: ActionType::Optimize,
                    packages: Vec::new(),
                    confidence: 0.85,
                    original_request: request.to_string(),
                    rollback_target: None,
                    optimize_flags: vec!["unused-packages".to_string(), "redundant-modules".to_string()],
                    suggestions: Vec::new(),
                });
            }
        }
        None
    }

    /// Parse suggestion requests
    fn parse_suggest(&self, request: &str) -> Option<AiAction> {
        let suggest_patterns = [
            r"(?:suggest|recommend)\s*(?:me)?\s*(?:some)?\s*(?:packages)?(?:for)?\s*(\w+)?",
            r"(?:what\s+(?:should|can\s+I)\s+(?:install|add))\s*(?:for)?\s*(\w+)?",
            r"(?:i\s+want\s+to)\s*(\w+)\s*(?:development|programming)?",
        ];

        for pattern in suggest_patterns {
            if let Some(caps) = Regex::new(pattern).ok()?.captures(request) {
                let category = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| "general".to_string());
                return Some(AiAction {
                    action: ActionType::Suggest,
                    packages: Vec::new(),
                    confidence: 0.8,
                    original_request: request.to_string(),
                    rollback_target: None,
                    optimize_flags: Vec::new(),
                    suggestions: Vec::new(), // Will be populated by handler
                });
            }
        }
        None
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
                        // Check if already installed
                        let already_installed = packages.iter().any(|p| {
                            self.context.as_ref().map_or(false, |ctx| ctx.is_installed(p))
                        });

                        let confidence = if already_installed {
                            0.6 // Lower confidence if already installed
                        } else {
                            0.8
                        };

                        return Some(AiAction {
                            action: ActionType::Install,
                            packages,
                            confidence,
                            original_request: request.to_string(),
                            rollback_target: None,
                            optimize_flags: Vec::new(),
                            suggestions: Vec::new(),
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
                            rollback_target: None,
                            optimize_flags: Vec::new(),
                            suggestions: Vec::new(),
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
                    rollback_target: None,
                    optimize_flags: Vec::new(),
                    suggestions: Vec::new(),
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

    /// Use LLM API for interpretation
    fn llm_interpret(&self, request: &str) -> Result<AiAction> {
        match &self.provider {
            LlmProvider::OpenAI => self.llm_openai(request),
            LlmProvider::Ollama { url } => self.llm_ollama(request, url),
            LlmProvider::OpenRouter => self.llm_openrouter(request),
        }
    }

    /// OpenAI API integration
    fn llm_openai(&self, request: &str) -> Result<AiAction> {
        let api_key = self
            .api_key
            .clone()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .context("No OpenAI API key configured. Set SLOP_AI_API_KEY or OPENAI_API_KEY")?;

        let api_url = std::env::var("SLOP_AI_API_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());

        self.call_llm_api(&api_url, &api_key, request, "gpt-3.5-turbo")
    }

    /// Ollama API integration (local LLM)
    fn llm_ollama(&self, request: &str, url: &str) -> Result<AiAction> {
        let model = std::env::var("SLOP_OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string());

        // Ollama uses a different API format
        let client = reqwest::blocking::Client::new();

        let prompt = format!(
            r#"You are a NixOS package management assistant. Convert this request into a JSON action.

Request: "{}"

Respond with ONLY valid JSON in this format:
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

        let response = client
            .post(format!("{}/api/generate", url.trim_end_matches('/')))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "prompt": prompt,
                "stream": false,
                "format": "json"
            }))
            .send()
            .context("Failed to send request to Ollama API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            bail!("Ollama API returned error ({}): {}", status, body);
        }

        #[derive(Deserialize)]
        struct OllamaResponse {
            response: String,
        }

        let ollama_response: OllamaResponse =
            response.json().context("Failed to parse Ollama response")?;

        self.parse_llm_json(&ollama_response.response)
    }

    /// OpenRouter API integration (multiple model providers)
    fn llm_openrouter(&self, request: &str) -> Result<AiAction> {
        let api_key = std::env::var("SLOP_OPENROUTER_KEY")
            .or_else(|_| std::env::var("OPENROUTER_API_KEY"))
            .context(
                "No OpenRouter API key configured. Set SLOP_OPENROUTER_KEY or OPENROUTER_API_KEY",
            )?;

        let model = std::env::var("SLOP_OPENROUTER_MODEL")
            .unwrap_or_else(|_| "meta-llama/llama-3.2-3b-instruct:free".to_string());

        let api_url = "https://openrouter.ai/api/v1/chat/completions";

        self.call_llm_api_with_model(api_url, &api_key, request, &model, Some("slop"))
    }

    /// Generic LLM API caller for OpenAI-compatible APIs
    fn call_llm_api(
        &self,
        api_url: &str,
        api_key: &str,
        request: &str,
        model: &str,
    ) -> Result<AiAction> {
        self.call_llm_api_with_model(api_url, api_key, request, model, None)
    }

    fn call_llm_api_with_model(
        &self,
        api_url: &str,
        api_key: &str,
        request: &str,
        model: &str,
        app_name: Option<&str>,
    ) -> Result<AiAction> {
        let prompt = format!(
            r#"You are a NixOS package management assistant. Convert this request into a JSON action.

Request: "{}"

Respond with ONLY valid JSON in this format:
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

        let mut client_builder = reqwest::blocking::Client::new()
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json");

        // Add OpenRouter-specific headers
        if let Some(app) = app_name {
            client_builder = client_builder
                .header("HTTP-Referer", "https://github.com/yourusername/slop")
                .header("X-Title", app);
        }

        let response = client_builder
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    {"role": "system", "content": "You are a helpful NixOS package assistant. Always respond with valid JSON only, no markdown formatting."},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.3,
                "max_tokens": 150
            }))
            .send()
            .context("Failed to send request to LLM API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            bail!("LLM API returned error ({}): {}", status, body);
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

        let llm_response: LlmResponse = response.json().context("Failed to parse LLM response")?;

        let content = llm_response
            .choices
            .first()
            .map(|c| c.message.content.as_str())
            .context("Empty LLM response")?;

        self.parse_llm_json(content)
    }

    /// Parse JSON response from LLM
    fn parse_llm_json(&self, content: &str) -> Result<AiAction> {
        // Extract JSON from response (might be wrapped in markdown or have extra text)
        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: serde_json::Value = serde_json::from_str(json_str)
            .with_context(|| format!("Failed to parse LLM JSON: {}", content))?;

        let action_str = parsed["action"].as_str().unwrap_or("search");
        let packages = parsed["packages"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
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
            original_request: String::new(), // Will be set by caller
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
        assert!(action
            .packages
            .iter()
            .any(|p| p.contains("nvim") || p.contains("vim")));
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
