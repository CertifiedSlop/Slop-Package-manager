//! Integration tests for slop
//!
//! End-to-end tests that verify the full workflow of package management operations.

use slop::ai_interpreter::{ActionType, AiInterpreter};
use slop::flake_manager::Flake;
use slop::nix_config::NixConfig;
use slop::package_resolver::PackageResolver;
use slop::rebuild::RebuildExecutor;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test configuration file
fn create_test_config(path: &std::path::Path, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

/// Helper to create a test flake file
fn create_test_flake(path: &std::path::Path, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

// ============================================================================
// NixConfig Integration Tests
// ============================================================================

#[test]
fn test_config_load_and_modify() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("configuration.nix");

    let initial_content = r#"{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  environment.systemPackages = with pkgs; [
    firefox
    neovim
    git
  ];
}
"#;

    create_test_config(&config_path, initial_content).expect("Failed to create test config");

    // Load config
    let mut config = NixConfig::load(&config_path).expect("Failed to load config");

    // Verify initial state
    assert!(config.has_package("firefox"));
    assert!(config.has_package("neovim"));
    assert!(config.has_package("git"));
    assert!(!config.has_package("vscode"));

    // Add a package
    config.add_package("vscode").expect("Failed to add package");
    assert!(config.has_package("vscode"));

    // Remove a package
    let removed = config
        .remove_package("git")
        .expect("Failed to remove package");
    assert!(removed);
    assert!(!config.has_package("git"));

    // Save and reload
    config.save().expect("Failed to save config");

    let reloaded = NixConfig::load(&config_path).expect("Failed to reload config");
    assert!(reloaded.has_package("vscode"));
    assert!(!reloaded.has_package("git"));
}

#[test]
fn test_config_backup_and_restore() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("configuration.nix");

    let initial_content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
  ];
}
"#;

    create_test_config(&config_path, initial_content).expect("Failed to create test config");
    let config = NixConfig::load(&config_path).expect("Failed to load config");

    // Create backup
    let backup_path = config.backup().expect("Failed to create backup");
    assert!(backup_path.exists());

    // Modify config
    let mut config = NixConfig::load(&config_path).expect("Failed to load config");
    config.add_package("neovim").expect("Failed to add package");
    config.save().expect("Failed to save config");

    // Verify backup has original content
    let backup_content = fs::read_to_string(&backup_path).expect("Failed to read backup");
    assert!(backup_content.contains("firefox"));
    assert!(!backup_content.contains("neovim"));
}

// ============================================================================
// PackageResolver Integration Tests
// ============================================================================

#[test]
fn test_package_resolution_chain() {
    let resolver = PackageResolver::new();

    // Test direct resolution
    assert_eq!(resolver.resolve("firefox"), Some("firefox"));

    // Test alias resolution
    assert_eq!(resolver.resolve("nvim"), Some("neovim"));
    assert_eq!(resolver.resolve("browser"), Some("firefox"));
    assert_eq!(resolver.resolve("editor"), Some("neovim"));

    // Test category resolution
    assert_eq!(resolver.resolve("terminal"), Some("alacritty"));
    assert_eq!(resolver.resolve("shell"), Some("zsh"));
}

#[test]
fn test_package_search_integration() {
    let resolver = PackageResolver::new();

    // Search for editors
    let results = resolver.search("editor");
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.attr_name == "neovim"));

    // Search for browsers
    let results = resolver.search("browser");
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.attr_name == "firefox"));

    // Search should return multiple results
    let results = resolver.search("git");
    assert!(!results.is_empty());
}

#[test]
fn test_package_suggestions() {
    let resolver = PackageResolver::new();

    // Test typo suggestions
    let suggestions = resolver.suggest("firef");
    assert!(suggestions.contains(&"firefox".to_string()));

    let suggestions = resolver.suggest("neo");
    assert!(suggestions
        .iter()
        .any(|s| s.contains("neovim") || s.contains("nvim")));
}

// ============================================================================
// AI Interpreter Integration Tests
// ============================================================================

#[test]
fn test_ai_install_interpretation() {
    let resolver = PackageResolver::new();
    let interpreter = AiInterpreter::new(resolver);

    // Test various install requests
    let action = interpreter.interpret("install firefox").unwrap();
    assert_eq!(action.action, ActionType::Install);
    assert!(action.packages.iter().any(|p| p.contains("firefox")));

    let action = interpreter.interpret("i need a text editor").unwrap();
    assert_eq!(action.action, ActionType::Install);
    assert!(!action.packages.is_empty());

    let action = interpreter.interpret("add git to my system").unwrap();
    assert_eq!(action.action, ActionType::Install);
    assert!(action.packages.iter().any(|p| p.contains("git")));
}

#[test]
fn test_ai_remove_interpretation() {
    let resolver = PackageResolver::new();
    let interpreter = AiInterpreter::new(resolver);

    // Test remove requests
    let action = interpreter.interpret("remove firefox").unwrap();
    assert_eq!(action.action, ActionType::Remove);
    assert!(action.packages.iter().any(|p| p.contains("firefox")));

    let action = interpreter.interpret("uninstall neovim").unwrap();
    assert_eq!(action.action, ActionType::Remove);
    assert!(action
        .packages
        .iter()
        .any(|p| p.contains("neovim") || p.contains("nvim")));
}

#[test]
fn test_ai_confidence_levels() {
    let resolver = PackageResolver::new();
    let interpreter = AiInterpreter::new(resolver);

    // Direct commands should have high confidence
    let action = interpreter.interpret("install firefox").unwrap();
    assert!(action.confidence >= 0.7);

    // Natural language should still have good confidence
    let action = interpreter
        .interpret("i would like to install a browser")
        .unwrap();
    assert!(action.confidence >= 0.7);
}

// ============================================================================
// Flake Manager Integration Tests
// ============================================================================

#[test]
fn test_flake_load_and_modify() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let flake_path = temp_dir.path().join("flake.nix");

    let initial_content = r#"{
  description = "My NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: { };
}
"#;

    create_test_flake(&flake_path, initial_content).expect("Failed to create test flake");

    // Load flake
    let mut flake = Flake::load(&flake_path).expect("Failed to load flake");

    // Verify initial state
    assert_eq!(
        flake.description,
        Some("My NixOS configuration".to_string())
    );
    assert_eq!(flake.inputs.len(), 2);
    assert!(flake.has_input("nixpkgs"));
    assert!(flake.has_input("flake-utils"));

    // Add input
    flake
        .add_input("home-manager", "github:nix-community/home-manager")
        .expect("Failed to add input");
    assert!(flake.has_input("home-manager"));
    assert_eq!(flake.inputs.len(), 3);

    // Remove input
    let removed = flake
        .remove_input("flake-utils")
        .expect("Failed to remove input");
    assert!(removed);
    assert!(!flake.has_input("flake-utils"));
    assert_eq!(flake.inputs.len(), 2);

    // Save and reload
    flake.save().expect("Failed to save flake");

    let reloaded = Flake::load(&flake_path).expect("Failed to reload flake");
    assert!(reloaded.has_input("home-manager"));
    assert!(!reloaded.has_input("flake-utils"));
}

#[test]
fn test_flake_input_update() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let flake_path = temp_dir.path().join("flake.nix");

    let initial_content = r#"{
  description = "Test flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: { };
}
"#;

    create_test_flake(&flake_path, initial_content).expect("Failed to create test flake");

    let mut flake = Flake::load(&flake_path).expect("Failed to load flake");

    // Update input URL
    let updated = flake
        .update_input("nixpkgs", "github:nixos/nixpkgs/nixos-23.11")
        .expect("Failed to update input");
    assert!(updated);
    assert!(flake.content.contains("github:nixos/nixpkgs/nixos-23.11"));

    // Save and verify
    flake.save().expect("Failed to save flake");

    let reloaded = Flake::load(&flake_path).expect("Failed to reload flake");
    assert!(reloaded
        .content
        .contains("github:nixos/nixpkgs/nixos-23.11"));
}

#[test]
fn test_flake_exists_check() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let flake_path = temp_dir.path().join("flake.nix");

    // Should not exist initially
    assert!(!Flake::exists(&flake_path));

    // Create flake
    create_test_flake(&flake_path, "{}").expect("Failed to create test flake");

    // Should exist now
    assert!(Flake::exists(&flake_path));
}

// ============================================================================
// Rebuild Executor Integration Tests
// ============================================================================

#[test]
fn test_rebuild_executor_dry_run() {
    let executor = RebuildExecutor::new(true, false, true);

    // Verify dry_run mode is enabled
    assert!(executor.is_dry_run());
    assert!(!executor.is_verbose());
    assert!(executor.is_interactive());

    // Dry run should not fail
    let result = executor.rebuild();
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.success);
}

#[test]
fn test_rebuild_executor_show_diff() {
    let executor = RebuildExecutor::new(false, false, false);

    let old_packages = vec!["firefox".to_string(), "git".to_string()];
    let new_packages = vec![
        "firefox".to_string(),
        "git".to_string(),
        "neovim".to_string(),
    ];

    // This test just verifies the method doesn't panic
    // In a real scenario, you'd capture stdout to verify output
    executor.show_diff(&old_packages, &new_packages);
}

#[test]
fn test_rebuild_executor_confirm_non_interactive() {
    let executor = RebuildExecutor::new(false, false, false);

    // Non-interactive mode should always return true
    let result = executor.confirm("Test confirmation");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

// ============================================================================
// End-to-End Workflow Tests
// ============================================================================

#[test]
fn test_full_install_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("configuration.nix");

    let initial_content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
  ];
}
"#;

    create_test_config(&config_path, initial_content).expect("Failed to create test config");

    // Simulate install workflow
    let mut config = NixConfig::load(&config_path).expect("Failed to load config");
    let resolver = PackageResolver::new();

    // Resolve package
    let package = resolver.resolve("neovim").unwrap();

    // Check if already installed
    if !config.has_package(package) {
        // Backup
        let _backup = config.backup().expect("Failed to backup");

        // Add package
        config.add_package(package).expect("Failed to add package");

        // Save
        config.save().expect("Failed to save config");

        // Verify
        let reloaded = NixConfig::load(&config_path).expect("Failed to reload config");
        assert!(reloaded.has_package("neovim"));
    }
}

#[test]
fn test_full_ai_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("configuration.nix");

    let initial_content = r#"{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim
  ];
}
"#;

    create_test_config(&config_path, initial_content).expect("Failed to create test config");

    // Simulate AI workflow
    let resolver = PackageResolver::new();
    let interpreter = AiInterpreter::new(resolver.clone());

    // Interpret AI request - use a simple direct request
    let action = interpreter.interpret("install neovim").unwrap();

    assert_eq!(action.action, ActionType::Install);
    assert!(!action.packages.is_empty());

    // Execute the action
    let mut config = NixConfig::load(&config_path).expect("Failed to load config");

    for package in &action.packages {
        if !config.has_package(package) {
            config.add_package(package).expect("Failed to add package");
        }
    }

    config.save().expect("Failed to save config");

    // Verify changes
    let reloaded = NixConfig::load(&config_path).expect("Failed to reload config");
    assert!(action.packages.iter().any(|p| reloaded.has_package(p)));
}

#[test]
fn test_full_flake_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let flake_path = temp_dir.path().join("flake.nix");

    let initial_content = r#"{
  description = "My system";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: { };
}
"#;

    create_test_flake(&flake_path, initial_content).expect("Failed to create test flake");

    // Simulate flake management workflow
    let mut flake = Flake::load(&flake_path).expect("Failed to load flake");

    // Add multiple inputs
    let inputs_to_add = vec![
        ("home-manager", "github:nix-community/home-manager"),
        ("flake-utils", "github:numtide/flake-utils"),
        ("nixpkgs-wayland", "github:nix-community/nixpkgs-wayland"),
    ];

    for (name, url) in &inputs_to_add {
        if !flake.has_input(name) {
            flake.add_input(name, url).expect("Failed to add input");
        }
    }

    // Save changes
    flake.save().expect("Failed to save flake");

    // Verify all inputs were added
    let reloaded = Flake::load(&flake_path).expect("Failed to reload flake");
    for (name, _) in &inputs_to_add {
        assert!(reloaded.has_input(name));
    }
}
