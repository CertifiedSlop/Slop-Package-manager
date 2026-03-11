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
    },

    /// Process a natural language request
    Ai {
        /// Natural language description of what you want
        request: String,
    },

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
