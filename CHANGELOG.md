# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Integration tests for end-to-end workflow validation
- Example configurations in `examples/` directory
- Package alias request issue template
- Roadmap feature voting issue template
- Documentation improvement issue template
- Issue template configuration for better routing
- Flake management commands (add, remove, update, lock, list)
- Shell completion generation command

### Changed
- Improved issue templates organization
- Enhanced documentation with comprehensive examples

### Fixed
- N/A

### Security
- N/A

---

## [0.1.0] - 2024-01-01

### Added
- Initial release of slop
- Core functionality for package management on NixOS
  - `slop install` - Install packages with automatic config editing
  - `slop remove` - Remove packages from configuration
  - `slop search` - Search for packages in nixpkgs
  - `slop ai` - Natural language package management
  - `slop list` - List installed packages
  - `slop diff` - Preview pending changes
- AI-powered natural language commands
  - Pattern matching for offline usage
  - LLM API support (OpenAI, Ollama, OpenRouter)
  - Context-aware package resolution
- Safe editing of configuration.nix
  - Automatic timestamped backups
  - Syntax validation with `nix-instantiate`
  - Interactive confirmation prompts
  - Dry-run mode for previewing changes
- Support for flakes (detection and management)
- Package search functionality with `nix-locate` integration
- Built-in package aliases (50+ common packages)
  - Browser aliases (firefox, chrome, chromium, brave)
  - Editor aliases (neovim, vim, emacs, vscode)
  - Terminal aliases (alacritty, kitty, wezterm)
  - Shell aliases (zsh, fish, bash)
  - Development tools (rust, nodejs, go, python)
  - Utilities (htop, btop, tree, ripgrep, fzf, bat)
- Safety features
  - Configuration backups before changes
  - Syntax validation before saving
  - Dry-run mode
  - Interactive confirmations
- Beautiful UX
  - Colored output with `colored` crate
  - Diff display with `similar` crate
  - Interactive prompts with `dialoguer`
- Comprehensive documentation
  - README.md with usage examples
  - Wiki with detailed guides
  - Contributing guidelines
  - Security policy
  - Code of conduct
- Development infrastructure
  - Dev container configuration
  - GitHub Actions CI/CD workflows
  - Shell completions (bash, zsh, fish, elvish, powershell)
  - Nix flake for easy installation
- Logging and debugging
  - `tracing` integration
  - Verbose mode with `--verbose` flag
  - Debug-level logging support

### Changed
- N/A

### Fixed
- N/A

### Security
- Automatic configuration backups
- No sensitive data logging
- Environment variable support for API keys

---

## Legend

- **Added** - New features
- **Changed** - Changes in existing functionality
- **Deprecated** - Soon-to-be removed features
- **Fixed** - Bug fixes
- **Removed** - Removed features
- **Security** - Security improvements
