# Example Configurations for slop

This directory contains example configuration files to help you get started with slop and understand how it modifies your NixOS configuration.

## Table of Contents

- [Basic Configuration](#basic-configuration)
- [Flake-based Configuration](#flake-based-configuration)
- [Home-Manager Integration](#home-manager-integration)
- [Before/After Examples](#beforeafter-examples)

---

## Basic Configuration

### Minimal `configuration.nix`

A simple starting point for new NixOS users:

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  # Boot configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Networking
  networking.hostName = "nixos";
  networking.networkmanager.enable = true;

  # System packages
  environment.systemPackages = with pkgs; [
    vim
    wget
    git
  ];

  # User configuration
  users.users.alice = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ];
  };

  # Enable sudo
  security.sudo.wheelNeedsPassword = true;

  system.stateVersion = "23.11";
}
```

### After Running `slop install firefox neovim alacritty`

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  # Boot configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Networking
  networking.hostName = "nixos";
  networking.networkmanager.enable = true;

  # System packages
  environment.systemPackages = with pkgs; [
    vim wget git firefox neovim alacritty
  ];

  # User configuration
  users.users.alice = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ];
  };

  # Enable sudo
  security.sudo.wheelNeedsPassword = true;

  system.stateVersion = "23.11";
}
```

---

## Flake-based Configuration

### `flake.nix` with slop integration

```nix
{
  description = "My NixOS Configuration with slop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
      ];
    };
  };
}
```

### `configuration.nix` for flakes

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  # System packages managed by slop
  environment.systemPackages = with pkgs; [
    vim
    git
  ];

  # Nix settings for flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  system.stateVersion = "23.11";
}
```

### After Running `slop flake add home-manager github:nix-community/home-manager`

```nix
{
  description = "My NixOS Configuration with slop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    home-manager.url = "github:nix-community/home-manager";
  };

  outputs = { self, nixpkgs, flake-utils }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
      ];
    };
  };
}
```

---

## Home-Manager Integration

### Configuration with home-manager module

```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  environment.systemPackages = with pkgs; [
    firefox
    neovim
    git
  ];

  # Home-Manager configuration
  home-manager.users.alice = { pkgs, ... }: {
    home.packages = with pkgs; [
      # User-specific packages
      discord
      spotify
    ];

    programs.git = {
      enable = true;
      userName = "Alice";
      userEmail = "alice@example.com";
    };

    programs.alacritty.enable = true;

    home.stateVersion = "23.11";
  };

  system.stateVersion = "23.11";
}
```

---

## Before/After Examples

### Example 1: Fresh Installation

**Before (empty packages list):**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
  ];
}
```

**After `slop install firefox neovim git htop`:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox neovim git htop
  ];
}
```

---

### Example 2: Adding Development Tools

**Before:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim
    git
  ];
}
```

**After `slop install rust nodejs go`:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim git rustup nodejs go
  ];
}
```

---

### Example 3: Removing Packages

**Before:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox
    chrome
    vlc
    discord
  ];
}
```

**After `slop remove chrome discord`:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    firefox vlc
  ];
}
```

---

### Example 4: AI Command Example

**Before:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim
  ];
}
```

**After `slop ai "set me up for rust development"`:**
```nix
{ config, pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    vim rustup cargo clippy rustfmt
  ];
}
```

---

## Usage Tips

### 1. Always Review Changes

Before applying changes, use dry-run mode:

```bash
slop --dry-run install firefox
```

### 2. Keep Backups

slop automatically creates backups, but you can also manually backup:

```bash
cp /etc/nixos/configuration.nix /etc/nixos/configuration.nix.backup
```

### 3. Use AI Commands for Complex Requests

```bash
slop ai "install a web browser and a terminal editor"
slop ai "set up my system for python development"
```

### 4. Manage Flake Inputs

```bash
# Add a new flake input
slop flake add home-manager github:nix-community/home-manager

# List all inputs
slop flake list

# Update all inputs
slop flake update
```

---

## Troubleshooting

### Configuration Syntax Error

If slop fails to parse your configuration, ensure:

1. Your `environment.systemPackages` section exists
2. The syntax follows standard Nix patterns
3. No unclosed braces or brackets

### Package Not Found

If a package can't be resolved:

1. Check the package name in [search.nixos.org](https://search.nixos.org/packages)
2. Use `slop search <query>` to find the correct attribute name
3. Try using the exact nixpkgs attribute name

### Flake Input Issues

If flake commands fail:

1. Ensure `flake.nix` exists in your configuration directory
2. Check that the input URL is valid
3. Run `nix flake update` manually if needed

---

## Additional Resources

- [NixOS Options Search](https://search.nixos.org/options)
- [NixOS Wiki](https://wiki.nixos.org/)
- [slop Documentation](https://github.com/CertifiedSlop/Slop-Package-manager/wiki)
- [Nixpkgs Manual](https://nixos.org/manual/nixpkgs/stable/)
