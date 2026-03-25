# slop Examples

> Practical examples and sample configurations for using slop

## Table of Contents

1. [Sample Configurations](#sample-configurations)
2. [Common Workflows](#common-workflows)
3. [AI Command Examples](#ai-command-examples)
4. [Flake Examples](#flake-examples)
5. [Before/After Comparisons](#beforeafter-comparisons)

---

## Sample Configurations

### Basic configuration.nix

**Minimal configuration before using slop:**

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "nixos";

  environment.systemPackages = with pkgs; [
    vim
    wget
    git
  ];

  system.stateVersion = "24.05";
}
```

**After running `slop install firefox neovim htop`:**

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "nixos";

  environment.systemPackages = with pkgs; [
    vim wget git firefox neovim htop
  ];

  system.stateVersion = "24.05";
}
```

### Configuration with Flakes

**flake.nix for development:**

```nix
{
  description = "My NixOS configuration with slop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    slop.url = "github:CertifiedSlop/Slop-Package-manager";
  };

  outputs = { self, nixpkgs, flake-utils, slop }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            slop.packages.${system}.default
            nixpkgs-fmt
          ];
        };
      }
    );
}
```

### Advanced configuration.nix

**With multiple package categories:**

```nix
{ config, pkgs, ... }: {
  imports = [
    ./hardware-configuration.nix
    ./audio.nix
    ./gaming.nix
  ];

  # System packages managed by slop
  environment.systemPackages = with pkgs; [
    # Development
    git
    neovim
    rustup
    python3
    nodejs

    # Browsers
    firefox
    chromium

    # Utilities
    htop
    btop
    tree
    ripgrep
    fd
    fzf
    bat
    eza

    # Media
    vlc
    mpv

    # Communication
    discord
    telegram-desktop
  ];

  # Development tools
  programs.git.enable = true;
  programs.neovim.enable = true;
  programs.zsh.enable = true;

  system.stateVersion = "24.05";
}
```

---

## Common Workflows

### Setting Up a New System

```bash
# 1. Run the interactive setup wizard
slop ai-setup

# 2. Install essential packages
slop install git neovim firefox

# 3. Set up for development
slop ai "set me up for Rust development"
slop ai "I need Python tools"

# 4. Install utilities
slop ai "install system monitoring tools"
slop ai "get me better command line tools"

# 5. Verify installation
slop list
slop ai-health
```

### Daily Package Management

```bash
# Install a new package
slop install vlc

# Remove unused package
slop remove telegram-desktop

# Search for a package
slop search "image editor"

# Preview before installing
slop --dry-run install gimp
```

### AI-Assisted Workflow

```bash
# Natural language installation
slop ai "I need a browser"
slop ai "install a terminal emulator"

# Get suggestions
slop ai-suggest rust
slop ai-suggest web-dev

# Check for issues
slop ai-check-conflicts
slop ai-health

# Optimize configuration
slop ai-optimize
```

---

## AI Command Examples

### Installation Requests

```bash
# Browser installation
slop ai "I need a web browser"
slop ai "install firefox"
slop ai "get me chrome"
slop ai "I want to browse the internet"

# Editor installation
slop ai "I need a text editor"
slop ai "install neovim"
slop ai "set me up with vscode"
slop ai "terminal editor please"

# Development setup
slop ai "I'm a Rust developer"
slop ai "set me up for Python"
slop ai "I need Node.js tools"
slop ai "golang development environment"

# Utility installation
slop ai "I need system monitoring"
slop ai "install htop"
slop ai "better ls command"
slop ai "fast file search"
```

### Removal Requests

```bash
slop ai "remove firefox"
slop ai "uninstall neovim"
slop ai "I don't want chrome anymore"
slop ai "get rid of discord"
```

### Rollback Requests

```bash
slop ai "undo my last change"
slop ai "revert the previous installation"
slop ai "rollback to the last generation"
slop ai "go back to generation 42"
```

### Optimization Requests

```bash
slop ai "optimize my configuration"
slop ai "remove unused packages"
slop ai "clean up my config"
slop ai "make my system smaller"
```

---

## Flake Examples

### Adding Flake Inputs

```bash
# Add home-manager
slop flake add home-manager --url "github:nix-community/home-manager"

# Add flake-utils
slop flake add flake-utils --url "github:numtide/flake-utils"

# Add nix-gaming
slop flake add nix-gaming --url "github:fufexan/nix-gaming"
```

### Updating Flake Inputs

```bash
# Update all inputs
slop flake update

# Update specific input
slop flake update nixpkgs
slop flake update home-manager

# Lock inputs
slop flake lock
```

### Sample flake.nix Evolution

**Before:**

```nix
{
  description = "My NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {};
}
```

**After running `slop flake add home-manager --url "github:nix-community/home-manager"`:**

```nix
{
  description = "My NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
  };

  outputs = { self, nixpkgs, home-manager }: {};
}
```

---

## Before/After Comparisons

### Example 1: Fresh Installation

**Before (empty packages list):**

```nix
environment.systemPackages = with pkgs; [
  vim
];
```

**Commands:**
```bash
slop install firefox neovim git
```

**After:**

```nix
environment.systemPackages = with pkgs; [
  vim firefox neovim git
];
```

### Example 2: Package Removal

**Before:**

```nix
environment.systemPackages = with pkgs; [
  vim firefox neovim git chromium discord
];
```

**Commands:**
```bash
slop remove chromium discord
```

**After:**

```nix
environment.systemPackages = with pkgs; [
  vim firefox neovim git
];
```

### Example 3: AI-Powered Setup

**Before:**

```nix
environment.systemPackages = with pkgs; [
  vim wget
];
```

**Commands:**
```bash
slop ai "set me up for web development"
```

**After (AI suggests and installs):**

```nix
environment.systemPackages = with pkgs; [
  vim wget nodejs npm vscode firefox git
];
```

---

## Shell Session Examples

### Complete Install Session

```bash
$ slop --dry-run install firefox
[DRY RUN] Would backup config
[DRY RUN] Would add 'firefox' to environment.systemPackages
[DRY RUN] Would run: sudo nixos-rebuild switch

$ slop install firefox
ℹ Backup created: "/etc/nixos/configuration.nix.bak.1711234567"
→ Would add 'firefox' to environment.systemPackages

Packages to install:
  + firefox

Apply changes and rebuild? [y/N]: y
✓ Configuration updated successfully
Running nixos-rebuild switch...
✓ System rebuild successful!
Generation: 42
```

### AI Session

```bash
$ slop ai "I need a browser"
🤖 Interpreted: Install (confidence: 85%)
Packages: firefox

Packages to install:
  + firefox

Apply changes and rebuild? [y/N]: y
✓ Configuration updated successfully
✓ System rebuild successful!

$ slop ai "now I need an editor"
🤖 Interpreted: Install (confidence: 80%)
Packages: neovim

Packages to install:
  + neovim

Apply changes and rebuild? [y/N]: y
✓ Configuration updated successfully
✓ System rebuild successful!
```

### Search Session

```bash
$ slop search editor
Found 5 package(s):

1. neovim Vim-based text editor
2. vim Improved vi clone
3. emacs Extensible text editor
4. vscode Code editor
5. nano Small and friendly editor

$ slop search --semantic "video editing"
Found 3 package(s):

1. kdenlive Video editor
2. obs-studio Streaming/recording software
3. pitivi Video editor
```

### Health Check Session

```bash
$ slop ai-health
🏥 Running system health check...

═══════════════════════════════════════
  System Health Report
═══════════════════════════════════════

✓ Disk Usage: 45% used (healthy)
✓ Configuration: Valid syntax
✓ Services: All running
✓ Security: Up to date

Overall: Your system looks healthy! 🎉
```

---

## Scripting Examples

### Batch Installation Script

```bash
#!/usr/bin/env bash
# install-dev-tools.sh

set -e

echo "Installing development tools..."

slop --yes install \
    git \
    neovim \
    rustup \
    python3 \
    nodejs \
    go \
    htop \
    btop \
    ripgrep \
    fd \
    fzf

echo "Development tools installed!"
slop list
```

### Configuration Backup Script

```bash
#!/usr/bin/env bash
# backup-config.sh

CONFIG_DIR="/etc/nixos"
BACKUP_DIR="$HOME/backups/nixos"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"

# Copy configuration
cp -r "$CONFIG_DIR" "$BACKUP_DIR/config-$TIMESTAMP"

# List current packages
slop list > "$BACKUP_DIR/packages-$TIMESTAMP.txt"

echo "Backup completed: $BACKUP_DIR/config-$TIMESTAMP"
```

---

## Troubleshooting Examples

### Fixing a Broken Configuration

```bash
# Check what went wrong
slop ai-health

# View the diff of pending changes
slop diff

# If config is broken, restore from backup
cp /etc/nixos/configuration.nix.bak.1711234567 /etc/nixos/configuration.nix

# Rebuild to restore
sudo nixos-rebuild switch
```

### Resolving Conflicts

```bash
# Check for conflicts
slop ai-check-conflicts

# Output might show:
# ⚠️ Multiple browsers detected:
#    - firefox
#    - chromium
#    - brave
# 💡 Consider keeping just one browser

# Remove duplicates
slop remove chromium brave
```

---

<div align="center">

**For more examples, see [USAGE.md](USAGE.md) and [README.md](README.md)**

</div>
