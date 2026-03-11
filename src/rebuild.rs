//! Rebuild executor for NixOS
//!
//! Handles running nixos-rebuild and related commands safely.

use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// Result of a rebuild operation
#[derive(Debug, Clone)]
pub struct RebuildResult {
    pub success: bool,
    pub output: String,
    pub generation: Option<u32>,
}

/// Executor for nixos-rebuild commands
pub struct RebuildExecutor {
    dry_run: bool,
    verbose: bool,
    interactive: bool,
}

impl RebuildExecutor {
    pub fn new(dry_run: bool, verbose: bool, interactive: bool) -> Self {
        RebuildExecutor {
            dry_run,
            verbose,
            interactive,
        }
    }

    /// Run nixos-rebuild switch
    pub fn rebuild(&self) -> Result<RebuildResult> {
        if self.dry_run {
            return self.dry_run_rebuild();
        }

        println!("{}", "Running nixos-rebuild switch...".bold().blue());

        let mut cmd = Command::new("sudo");
        cmd.arg("nixos-rebuild")
            .arg("switch")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if self.verbose {
            cmd.arg("--show-trace");
        }

        let output = cmd.output().context("Failed to execute nixos-rebuild")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let full_output = format!("{}\n{}", stdout, stderr);

        // Extract generation number if present
        let generation = self.extract_generation(&full_output);

        if output.status.success() {
            println!("{}", "✓ System rebuild successful!".green().bold());
            if let Some(gen) = generation {
                println!(
                    "{} {}",
                    "Generation:".green(),
                    gen.to_string().green().bold()
                );
            }
        } else {
            eprintln!("{}", "✗ Rebuild failed!".red().bold());
            eprintln!("{}", full_output);
        }

        Ok(RebuildResult {
            success: output.status.success(),
            output: full_output,
            generation,
        })
    }

    /// Dry run - show what would happen
    fn dry_run_rebuild(&self) -> Result<RebuildResult> {
        println!(
            "{}",
            "[DRY RUN] Would execute: sudo nixos-rebuild switch"
                .yellow()
                .bold()
        );
        println!(
            "{}",
            "[DRY RUN] No changes will be applied to the system.".yellow()
        );

        // Check if nixos-rebuild is available
        let check = Command::new("nixos-rebuild").arg("--version").output();

        match check {
            Ok(out) => {
                let version = String::from_utf8_lossy(&out.stdout);
                println!(
                    "{} {}",
                    "[DRY RUN] nixos-rebuild available:".green(),
                    version.trim()
                );
            }
            Err(_) => {
                println!(
                    "{}",
                    "[DRY RUN] Warning: nixos-rebuild not found in PATH".yellow()
                );
            }
        }

        Ok(RebuildResult {
            success: true,
            output: String::from("Dry run completed"),
            generation: None,
        })
    }

    /// Extract generation number from rebuild output
    fn extract_generation(&self, output: &str) -> Option<u32> {
        // Look for patterns like "Done. The new configuration is generation 123"
        for line in output.lines() {
            if line.contains("generation") {
                for word in line.split_whitespace() {
                    if let Ok(num) = word.parse::<u32>() {
                        return Some(num);
                    }
                }
            }
        }
        None
    }

    /// Validate the configuration without rebuilding
    pub fn check(&self, config_path: &Path) -> Result<bool> {
        println!("{}", "Checking Nix configuration...".blue());

        let output = Command::new("nix-instantiate")
            .arg("--check")
            .arg(config_path)
            .output()
            .context("Failed to run nix-instantiate")?;

        if output.status.success() {
            println!("{}", "✓ Configuration is valid".green());
            Ok(true)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", "✗ Configuration check failed:".red());
            eprintln!("{}", stderr);
            Ok(false)
        }
    }

    /// Show what packages would be added/removed
    pub fn show_diff(&self, old_packages: &[String], new_packages: &[String]) {
        let added: Vec<_> = new_packages
            .iter()
            .filter(|p| !old_packages.contains(p))
            .collect();

        let removed: Vec<_> = old_packages
            .iter()
            .filter(|p| !new_packages.contains(p))
            .collect();

        if !added.is_empty() {
            println!("\n{}", "Packages to install:".green().bold());
            for pkg in &added {
                println!("  {} {}", "+".green(), pkg.green());
            }
        }

        if !removed.is_empty() {
            println!("\n{}", "Packages to remove:".red().bold());
            for pkg in &removed {
                println!("  {} {}", "-".red(), pkg.red());
            }
        }

        if added.is_empty() && removed.is_empty() {
            println!("{}", "No changes to apply.".yellow());
        }
    }

    /// Prompt for confirmation
    pub fn confirm(&self, message: &str) -> Result<bool> {
        if !self.interactive {
            return Ok(true);
        }

        if self.dry_run {
            return Ok(false);
        }

        print!("{} [y/N]: ", message.yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        Ok(input == "y" || input == "yes")
    }

    /// Run nixos-rebuild with custom arguments
    pub fn rebuild_with_args(&self, args: &[&str]) -> Result<RebuildResult> {
        if self.dry_run {
            println!(
                "{}",
                format!(
                    "[DRY RUN] Would execute: sudo nixos-rebuild {}",
                    args.join(" ")
                )
                .yellow()
            );
            return Ok(RebuildResult {
                success: true,
                output: String::from("Dry run"),
                generation: None,
            });
        }

        let mut cmd = Command::new("sudo");
        cmd.arg("nixos-rebuild");

        for arg in args {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().context("Failed to execute nixos-rebuild")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let full_output = format!("{}\n{}", stdout, stderr);
        let generation = self.extract_generation(&full_output);

        Ok(RebuildResult {
            success: output.status.success(),
            output: full_output,
            generation,
        })
    }
}

/// Check if running on NixOS
pub fn is_nixos() -> bool {
    Path::new("/etc/NIXOS").exists()
}

/// Check if user has sudo privileges
pub fn has_sudo() -> bool {
    Command::new("sudo")
        .arg("-n")
        .arg("true")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_generation() {
        let executor = RebuildExecutor::new(false, false, true);
        let output = "building NixOS...
Done. The new configuration is generation 42";

        assert_eq!(executor.extract_generation(output), Some(42));
    }

    #[test]
    fn test_show_diff() {
        let executor = RebuildExecutor::new(false, false, true);
        let old = vec!["firefox".to_string(), "git".to_string()];
        let new = vec![
            "firefox".to_string(),
            "git".to_string(),
            "neovim".to_string(),
        ];

        // This just tests that it doesn't panic
        executor.show_diff(&old, &new);
    }
}
