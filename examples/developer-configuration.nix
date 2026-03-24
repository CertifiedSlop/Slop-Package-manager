# Developer NixOS Configuration
#
# This configuration is optimized for software development.
# Includes common development tools and environments.

{ config, pkgs, ... }: {

  imports = [ ./hardware-configuration.nix ];

  # Boot configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Networking
  networking.hostName = "devbox";
  networking.networkmanager.enable = true;

  # Development tools
  environment.systemPackages = with pkgs; [
    # Core tools
    git
    vim
    neovim
    wget
    curl

    # Rust development
    rustup
    cargo
    rustfmt
    clippy

    # Web development
    nodejs
    nodePackages.npm
    nodePackages.yarn

    # Python development
    python3
    python3Packages.pip
    python3Packages.venv

    # Go development
    go

    # Database tools
    postgresql
    mysql
    sqlite

    # Container tools
    docker
    docker-compose

    # Terminal utilities
    tmux
    htop
    btop
    ripgrep
    fd
    fzf
    bat
    eza

    # Network tools
    curl
    wget
    httpie
    jq
  ];

  # Enable Docker
  virtualisation.docker.enable = true;
  users.users.alice.extraGroups = [ "docker" ];

  # Enable flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nix.settings.auto-optimise-store = true;

  # User configuration
  users.users.alice = {
    isNormalUser = true;
    description = "Developer";
    extraGroups = [ "wheel" "networkmanager" "docker" ];
    shell = pkgs.zsh;
  };

  # Zsh configuration
  programs.zsh.enable = true;

  # State version
  system.stateVersion = "23.11";

}
