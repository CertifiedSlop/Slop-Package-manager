//! CLI argument parser for slop
//!
//! Defines the command-line interface using clap.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "slop")]
#[command(author = "Slop Team")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered package manager for NixOS", long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Path to configuration.nix (default: /etc/nixos/configuration.nix)
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Enable dry-run mode (no changes applied)
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    /// Skip confirmation prompts
    #[arg(short, long, global = true)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a package by name
    Install {
        /// Package name to install
        package: String,
    },

    /// Remove a package by name
    Remove {
        /// Package name to remove
        package: String,
    },

    /// Search for packages
    Search {
        /// Search query
        query: String,

        /// Use semantic search
        #[arg(short, long)]
        semantic: bool,
    },

    /// Process a natural language request
    Ai {
        /// Natural language description of what you want
        request: String,
    },

    /// Get AI-powered package suggestions
    AiSuggest {
        /// Category or use case (e.g., "rust", "web-dev", "gaming")
        category: Option<String>,
    },

    /// Optimize your configuration with AI
    AiOptimize {
        /// Show only suggestions without making changes
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Interactive AI chat mode
    AiChat,

    /// Run the interactive setup wizard
    AiSetup,

    /// Detect hardware and get recommendations
    AiDetectHardware,

    /// Check for package conflicts
    AiCheckConflicts {
        /// Packages to check
        packages: Vec<String>,
    },

    /// Run AI system health check
    AiHealth,

    /// Show conversation history
    AiHistory {
        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Clear AI conversation history
    AiClearHistory,

    /// Show current installed packages
    List,

    /// Show pending changes as a diff
    Diff {
        /// Package to add (optional, for preview)
        #[arg(short, long)]
        add: Option<String>,

        /// Package to remove (optional, for preview)
        #[arg(short, long)]
        remove: Option<String>,
    },

    /// Update packages or flake inputs
    Update {
        /// Update flake inputs instead of packages
        #[arg(short, long)]
        flake: bool,

        /// Specific input to update (for flake updates)
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Manage flake inputs
    Flake {
        #[command(subcommand)]
        action: FlakeCommands,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for (bash, elvish, fish, powershell, zsh)
        #[arg(short, long)]
        shell: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum FlakeCommands {
    /// Add a new flake input
    Add {
        /// Input name
        name: String,

        /// Input URL (e.g., github:nixos/nixpkgs/nixos-unstable)
        #[arg(short, long)]
        url: String,
    },

    /// Remove a flake input
    Remove {
        /// Input name to remove
        name: String,
    },

    /// Update flake inputs
    Update {
        /// Specific input to update (updates all if not specified)
        name: Option<String>,
    },

    /// Lock flake inputs
    Lock,

    /// List flake inputs
    List,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_install() {
        let cli = Cli::parse_from(["slop", "install", "firefox"]);
        assert!(matches!(cli.command, Commands::Install { ref package } if package == "firefox"));
    }

    #[test]
    fn test_parse_remove() {
        let cli = Cli::parse_from(["slop", "remove", "neovim"]);
        assert!(matches!(cli.command, Commands::Remove { ref package } if package == "neovim"));
    }

    #[test]
    fn test_parse_ai() {
        let cli = Cli::parse_from(["slop", "ai", "install a browser"]);
        assert!(
            matches!(cli.command, Commands::Ai { ref request } if request == "install a browser")
        );
    }

    #[test]
    fn test_parse_dry_run() {
        let cli = Cli::parse_from(["slop", "--dry-run", "install", "git"]);
        assert!(cli.dry_run);
    }
}
