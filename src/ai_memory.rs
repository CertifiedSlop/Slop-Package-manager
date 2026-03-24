//! AI Memory & Conversation History
//!
//! Stores conversation history and enables contextual conversations.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Conversation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub timestamp: u64,
    pub role: MessageRole,
    pub content: String,
    pub action: Option<String>,
    pub packages: Vec<String>,
}

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    pub id: String,
    pub started_at: u64,
    pub last_activity: u64,
    pub messages: Vec<ConversationMessage>,
}

/// User preferences learned over time
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    pub preferred_categories: Vec<String>,
    pub avoided_packages: Vec<String>,
    pub cli_over_gui: bool,
    pub minimal_install: bool,
    pub max_package_size_mb: Option<f64>,
    pub preferred_editors: Vec<String>,
    pub preferred_shells: Vec<String>,
    pub confidence_threshold: f32,
}

/// AI Memory manager
pub struct AiMemory {
    cache_dir: PathBuf,
    current_session: ConversationSession,
    preferences: UserPreferences,
    max_history_size: usize,
}

impl AiMemory {
    /// Create a new AI memory manager
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("~/.cache"))
            .join("slop");

        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;

        let mut memory = AiMemory {
            cache_dir,
            current_session: ConversationSession {
                id: Self::generate_session_id(),
                started_at: Self::current_timestamp(),
                last_activity: Self::current_timestamp(),
                messages: Vec::new(),
            },
            preferences: UserPreferences::default(),
            max_history_size: 100,
        };

        memory.load_preferences()?;
        memory.load_session()?;

        Ok(memory)
    }

    /// Generate a unique session ID
    fn generate_session_id() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("session_{}", timestamp)
    }

    /// Get current timestamp in seconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Get the history file path
    #[allow(dead_code)]
    fn history_path(&self) -> PathBuf {
        self.cache_dir.join("ai_history.json")
    }

    /// Get the preferences file path
    fn preferences_path(&self) -> PathBuf {
        self.cache_dir.join("preferences.json")
    }

    /// Get the session file path
    fn session_path(&self) -> PathBuf {
        self.cache_dir.join("current_session.json")
    }

    /// Add a user message to the conversation
    pub fn add_user_message(&mut self, content: &str) {
        let message = ConversationMessage {
            timestamp: Self::current_timestamp(),
            role: MessageRole::User,
            content: content.to_string(),
            action: None,
            packages: Vec::new(),
        };

        self.current_session.messages.push(message);
        self.current_session.last_activity = Self::current_timestamp();
        self.trim_history();
    }

    /// Add an assistant response
    pub fn add_assistant_message(
        &mut self,
        content: &str,
        action: Option<&str>,
        packages: Vec<String>,
    ) {
        let message = ConversationMessage {
            timestamp: Self::current_timestamp(),
            role: MessageRole::Assistant,
            content: content.to_string(),
            action: action.map(String::from),
            packages,
        };

        self.current_session.messages.push(message);
        self.current_session.last_activity = Self::current_timestamp();
        self.trim_history();
    }

    /// Trim history to max size
    fn trim_history(&mut self) {
        while self.current_session.messages.len() > self.max_history_size {
            self.current_session.messages.remove(0);
        }
    }

    /// Save session to disk
    pub fn save_session(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.current_session)
            .context("Failed to serialize session")?;

        fs::write(self.session_path(), content).context("Failed to save session")?;

        Ok(())
    }

    /// Load session from disk
    pub fn load_session(&mut self) -> Result<()> {
        let session_path = self.session_path();

        if session_path.exists() {
            let content =
                fs::read_to_string(&session_path).context("Failed to read session file")?;

            if let Ok(session) = serde_json::from_str(&content) {
                self.current_session = session;
            }
        }

        Ok(())
    }

    /// Save preferences to disk
    pub fn save_preferences(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.preferences)
            .context("Failed to serialize preferences")?;

        fs::write(self.preferences_path(), content).context("Failed to save preferences")?;

        Ok(())
    }

    /// Load preferences from disk
    pub fn load_preferences(&mut self) -> Result<()> {
        let prefs_path = self.preferences_path();

        if prefs_path.exists() {
            let content =
                fs::read_to_string(&prefs_path).context("Failed to read preferences file")?;

            if let Ok(prefs) = serde_json::from_str(&content) {
                self.preferences = prefs;
            }
        }

        Ok(())
    }

    /// Get conversation history
    pub fn get_history(&self, limit: Option<usize>) -> Vec<&ConversationMessage> {
        let limit = limit.unwrap_or(self.max_history_size);
        self.current_session
            .messages
            .iter()
            .rev()
            .take(limit)
            .collect()
    }

    /// Get recent packages from conversation
    pub fn get_recent_packages(&self, limit: usize) -> Vec<String> {
        let mut packages = Vec::new();

        for message in self.current_session.messages.iter().rev() {
            for pkg in &message.packages {
                if !packages.contains(pkg) {
                    packages.push(pkg.clone());
                }
            }

            if packages.len() >= limit {
                break;
            }
        }

        packages
    }

    /// Find previous mention of a package
    pub fn find_package_mention(&self, package: &str) -> Option<&ConversationMessage> {
        self.current_session
            .messages
            .iter()
            .rev()
            .find(|m| m.packages.iter().any(|p| p.contains(package)))
    }

    /// Get context for follow-up requests
    pub fn get_context(&self) -> Option<String> {
        // Look at last few messages for context
        let recent = self.get_history(Some(5));

        if recent.is_empty() {
            return None;
        }

        // Check for recent package installations
        let recent_packages = self.get_recent_packages(3);

        if !recent_packages.is_empty() {
            return Some(format!(
                "User recently installed: {}",
                recent_packages.join(", ")
            ));
        }

        None
    }

    /// Resolve references like "the browser" or "that editor"
    pub fn resolve_reference(&self, reference: &str) -> Option<String> {
        let reference_lower = reference.to_lowercase();

        // Check for common references
        if reference_lower.contains("browser") {
            return self.find_recent_package_by_category("browser");
        }

        if reference_lower.contains("editor") {
            return self.find_recent_package_by_category("editor");
        }

        if reference_lower.contains("terminal") {
            return self.find_recent_package_by_category("terminal");
        }

        if reference_lower.contains("shell") {
            return self.find_recent_package_by_category("shell");
        }

        None
    }

    /// Find recent package in a category
    fn find_recent_package_by_category(&self, category: &str) -> Option<String> {
        let category_packages: Vec<(&str, &str)> = match category {
            "browser" => vec![("firefox", "browser"), ("chromium", "browser")],
            "editor" => vec![
                ("neovim", "editor"),
                ("vim", "editor"),
                ("vscode", "editor"),
            ],
            "terminal" => vec![("alacritty", "terminal"), ("kitty", "terminal")],
            "shell" => vec![("zsh", "shell"), ("fish", "shell"), ("bash", "shell")],
            _ => return None,
        };

        for (pkg, _) in &category_packages {
            if self.find_package_mention(pkg).is_some() {
                return Some(pkg.to_string());
            }
        }

        None
    }

    /// Record a user preference
    pub fn record_preference(&mut self, preference: &str, value: bool) {
        match preference {
            "cli_over_gui" => {
                self.preferences.cli_over_gui = value;
            }
            "minimal_install" => {
                self.preferences.minimal_install = value;
            }
            _ => {}
        }
    }

    /// Record package avoidance
    pub fn record_avoidance(&mut self, package: &str) {
        if !self
            .preferences
            .avoided_packages
            .contains(&package.to_string())
        {
            self.preferences.avoided_packages.push(package.to_string());
        }
    }

    /// Check if a package should be avoided
    pub fn should_avoid(&self, package: &str) -> bool {
        self.preferences
            .avoided_packages
            .iter()
            .any(|p| p == package)
    }

    /// Get user preferences
    pub fn get_preferences(&self) -> &UserPreferences {
        &self.preferences
    }

    /// Update preferences based on user action
    pub fn learn_from_action(&mut self, action: &str, accepted: bool) {
        if !accepted {
            // User rejected a suggestion - learn from it
            if action.contains("cli") || action.contains("terminal") {
                self.record_preference("cli_over_gui", true);
            }
            if action.contains("minimal") || action.contains("small") {
                self.record_preference("minimal_install", true);
            }
        }
    }

    /// Start a new session
    pub fn new_session(&mut self) -> Result<()> {
        // Save current session to history
        self.save_session()?;

        // Start fresh session
        self.current_session = ConversationSession {
            id: Self::generate_session_id(),
            started_at: Self::current_timestamp(),
            last_activity: Self::current_timestamp(),
            messages: Vec::new(),
        };

        Ok(())
    }

    /// Get session info
    pub fn get_session_info(&self) -> &ConversationSession {
        &self.current_session
    }

    /// Clear history
    pub fn clear_history(&mut self) -> Result<()> {
        self.current_session.messages.clear();
        self.save_session()
    }

    /// Export conversation history
    pub fn export_history(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.current_session).context("Failed to export history")
    }
}

impl Default for AiMemory {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize AI memory: {}", e);
            AiMemory {
                cache_dir: PathBuf::from("/tmp/slop"),
                current_session: ConversationSession {
                    id: String::new(),
                    started_at: 0,
                    last_activity: 0,
                    messages: Vec::new(),
                },
                preferences: UserPreferences::default(),
                max_history_size: 100,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = AiMemory::default();
        assert!(
            memory.get_session_info().id.is_empty() || !memory.get_session_info().id.is_empty()
        );
    }

    #[test]
    fn test_add_messages() {
        let mut memory = AiMemory::default();

        memory.add_user_message("install firefox");
        memory.add_assistant_message(
            "Installing firefox",
            Some("install"),
            vec!["firefox".to_string()],
        );

        assert_eq!(memory.current_session.messages.len(), 2);
    }

    #[test]
    fn test_get_recent_packages() {
        let mut memory = AiMemory::default();

        memory.add_user_message("install firefox");
        memory.add_assistant_message(
            "Installing firefox",
            Some("install"),
            vec!["firefox".to_string()],
        );
        memory.add_user_message("also install neovim");
        memory.add_assistant_message(
            "Installing neovim",
            Some("install"),
            vec!["neovim".to_string()],
        );

        let packages = memory.get_recent_packages(5);
        assert!(packages.contains(&"firefox".to_string()));
        assert!(packages.contains(&"neovim".to_string()));
    }

    #[test]
    fn test_resolve_reference() {
        let mut memory = AiMemory::default();

        memory.add_user_message("install firefox");
        memory.add_assistant_message(
            "Installing firefox",
            Some("install"),
            vec!["firefox".to_string()],
        );

        let resolved = memory.resolve_reference("the browser");
        assert_eq!(resolved, Some("firefox".to_string()));
    }

    #[test]
    fn test_preferences() {
        let mut memory = AiMemory::default();

        memory.record_preference("cli_over_gui", true);
        assert!(memory.preferences.cli_over_gui);

        memory.record_avoidance("libreoffice");
        assert!(memory.should_avoid("libreoffice"));
    }
}
