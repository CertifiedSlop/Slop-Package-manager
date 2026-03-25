# slop Usage Guide

> Comprehensive guide to using slop, the AI-powered package manager for NixOS

## Table of Contents

1. [Quick Start](#quick-start)
2. [Basic Commands](#basic-commands)
3. [AI-Powered Commands](#ai-powered-commands)
4. [Flake Management](#flake-management)
5. [Global Options](#global-options)
6. [Natural Language Examples](#natural-language-examples)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### First Time Usage

```bash
# Install your first package
slop install firefox

# Or use natural language
slop ai "I need a web browser"

# Preview changes before applying
slop --dry-run install neovim
```

### Recommended Workflow

1. **Preview changes** with `--dry-run`
2. **Review the diff** of what will be modified
3. **Apply changes** with confirmation prompt
4. **Verify** the system rebuild succeeded

---

## Basic Commands

### Install Packages

Add packages to `environment.systemPackages`:

```bash
# Install a single package
slop install firefox

# Install multiple packages
slop install neovim git htop

# Install with verbose output
slop --verbose install rustup

# Skip confirmation prompt
slop --yes install cargo
```

**What happens:**
1. Resolves package name (handles aliases like `nvim` → `neovim`)
2. Checks if already installed
3. Creates backup of configuration.nix
4. Adds package to config
5. Shows diff of changes
6. Asks for confirmation
7. Runs `sudo nixos-rebuild switch`

### Remove Packages

Remove packages from `environment.systemPackages`:

```bash
# Remove a single package
slop remove firefox

# Remove with verbose output
slop --verbose remove neovim
```

**What happens:**
1. Checks if package is installed
2. Creates backup of configuration.nix
3. Removes package from config
4. Shows diff of changes
5. Asks for confirmation
6. Runs `sudo nixos-rebuild switch`

### Search Packages

Search for packages in nixpkgs:

```bash
# Basic search
slop search editor

# Search with semantic understanding
slop search --semantic "video editing"

# Search for development tools
slop search rust
```

### List Installed Packages

```bash
# List all packages in environment.systemPackages
slop list
```

### Preview Changes

```bash
# Show what would change if you add a package
slop diff --add firefox

# Show what would change if you remove a package
slop diff --remove neovim
```

---

## AI-Powered Commands

### Natural Language Processing

The `ai` command interprets natural language requests:

```bash
# Install requests
slop ai "install a browser"
slop ai "I need a terminal emulator"
slop ai "set me up for Python development"

# Remove requests
slop ai "remove the web browser"
slop ai "uninstall neovim"

# Rollback requests
slop ai "undo my last change"
slop ai "rollback to the previous generation"

# Optimization requests
slop ai "optimize my configuration"
slop ai "remove unused packages"
```

### Interactive Setup Wizard

```bash
# Run the guided setup
slop ai-setup
```

The wizard will ask about:
- Your use case (development, desktop, server)
- Preferred tools (editor, terminal, shell)
- Development languages
- Desktop environment preferences

### AI Chat Mode

```bash
# Enter interactive chat
slop ai-chat
```

In chat mode, you can have multi-turn conversations:

```
❯ I want to set up for Rust development
🤖 Installing: rustup

❯ Now I need a good editor
🤖 Installing: neovim

❯ What about debugging tools?
🤖 Installing: lldb, gdb
```

Chat commands:
- `help` - Show available commands
- `quit` / `exit` - Exit chat mode

### Package Suggestions

```bash
# Get suggestions for a category
slop ai-suggest rust
slop ai-suggest python
slop ai-suggest gaming
slop ai-suggest web-dev

# Show all available categories
slop ai-suggest
```

### Configuration Optimization

```bash
# Analyze configuration for optimizations
slop ai-optimize

# Dry-run mode (show suggestions without changes)
slop ai-optimize --dry-run
```

### Hardware Detection

```bash
# Detect hardware and get recommendations
slop ai-detect-hardware
```

Detects:
- GPU (NVIDIA, AMD, Intel)
- Network adapters
- Audio devices
- Printers/scanners

### Conflict Detection

```bash
# Check for conflicts in current config
slop ai-check-conflicts

# Check specific packages before installing
slop ai-check-conflicts firefox chromium brave
```

### Health Check

```bash
# Run system health check
slop ai-health
```

Checks:
- Disk usage
- Configuration syntax
- Service status
- Security updates

### Conversation History

```bash
# View recent AI conversations
slop ai-history

# View last 20 entries
slop ai-history --limit 20

# Clear conversation history
slop ai-clear-history
```

### Semantic Search

```bash
# Search by use case instead of keywords
slop search --semantic "video editing"
slop search --semantic "system monitoring"
```

---

## Flake Management

Slop supports managing flake inputs:

```bash
# Add a flake input
slop flake add home-manager --url "github:nix-community/home-manager"

# Remove a flake input
slop flake remove old-input

# Update all inputs
slop flake update

# Update specific input
slop flake update nixpkgs

# Lock inputs
slop flake lock

# List inputs
slop flake list

# Update packages (non-flake)
slop update

# Update flake inputs
slop update --flake
slop update --flake --input nixpkgs
```

---

## Global Options

### Verbose Mode

```bash
slop --verbose install firefox
```

Shows detailed output including:
- Backup file location
- Package resolution steps
- Diff details
- Rebuild output

### Dry-Run Mode

```bash
slop --dry-run install firefox
slop --dry-run remove neovim
slop --dry-run ai "install a browser"
```

Preview actions without making changes:
- Shows what would be backed up
- Shows what would be added/removed
- Shows the rebuild command that would run

### Skip Confirmation

```bash
slop --yes install git
```

Skip interactive confirmation prompts. Useful for scripting.

### Custom Config Path

```bash
slop --config /path/to/configuration.nix install firefox
```

Use a custom configuration file path (default: `/etc/nixos/configuration.nix`).

### Combined Options

```bash
slop --verbose --dry-run --yes install firefox
```

---

## Natural Language Examples

### Browser Installation

```bash
slop ai "I need a browser"           # → firefox
slop ai "install chrome"             # → google-chrome
slop ai "get me firefox"             # → firefox
slop ai "I want to browse the web"   # → firefox
```

### Editor Installation

```bash
slop ai "I need an editor"           # → neovim
slop ai "install vim"                # → vim
slop ai "set me up with emacs"       # → emacs
slop ai "I want vscode"              # → vscode
slop ai "terminal editor"            # → neovim
```

### Development Setup

```bash
slop ai "set me up for rust"         # → rustup, cargo, rustfmt, clippy
slop ai "I'm a python developer"     # → python3, pip, venv
slop ai "I need nodejs"              # → nodejs, npm
slop ai "golang development"         # → go
```

### Utility Installation

```bash
slop ai "I need htop"                # → htop
slop ai "system monitor"             # → htop or btop
slop ai "better ls"                  # → eza
slop ai "I need ripgrep"             # → ripgrep
slop ai "find files fast"            # → fd
```

### Removal Requests

```bash
slop ai "remove firefox"             # → removes firefox
slop ai "uninstall neovim"           # → removes neovim
slop ai "I don't want chrome"        # → removes google-chrome
slop ai "get rid of vscode"          # → removes vscode
```

### Rollback Requests

```bash
slop ai "undo my last change"        # → rollback
slop ai "revert the previous change" # → rollback
slop ai "go back to generation 42"   # → rollback to gen 42
```

---

## Environment Variables

### AI Configuration

```bash
# OpenAI API
export SLOP_AI_API_KEY="sk-..."
export SLOP_AI_API_URL="https://api.openai.com/v1/chat/completions"

# Ollama (local LLM)
export SLOP_OLLAMA_URL="http://localhost:11434"
export SLOP_OLLAMA_MODEL="llama3.2"

# OpenRouter
export SLOP_OPENROUTER_KEY="sk-or-..."
export SLOP_OPENROUTER_MODEL="meta-llama/llama-3.2-3b-instruct:free"
```

### Shell Completions

```bash
# Generate bash completions
slop completions --shell bash > ~/.local/share/bash-completion/completions/slop

# Generate zsh completions
slop completions --shell zsh > ~/.zsh/completions/_slop

# Generate fish completions
slop completions --shell fish > ~/.config/fish/completions/slop.fish
```

---

## Troubleshooting

### Common Issues

**Config file not found:**
```bash
# Use custom config path
slop --config /etc/nixos/configuration.nix install firefox
```

**Permission denied:**
```bash
# Ensure you have write access to /etc/nixos/
# Or run with sudo (not recommended for regular use)
sudo slop install firefox
```

**nixos-rebuild not found:**
```bash
# This tool only works on NixOS
# Use --dry-run for testing on other systems
slop --dry-run install firefox
```

**AI commands not working:**
```bash
# AI commands work offline with pattern matching
# For LLM features, configure an API provider
export SLOP_OLLAMA_URL="http://localhost:11434"
```

**Package not found:**
```bash
# Search for the correct package name
slop search <query>

# Check suggestions
slop ai-suggest <category>
```

### Getting Help

```bash
# Show help
slop --help

# Show help for specific command
slop install --help
slop ai --help
```

---

<div align="center">

**For more information, see [README.md](README.md) and [CONTRIBUTING.md](CONTRIBUTING.md)**

</div>
