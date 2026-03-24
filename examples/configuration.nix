# Minimal NixOS Configuration for slop
#
# This is a basic configuration.nix file that works with slop.
# Copy this to /etc/nixos/configuration.nix to get started.

{ config, pkgs, ... }: {

  # Include hardware configuration
  imports = [ ./hardware-configuration.nix ];

  # Boot loader (adjust for your system)
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # File system
  fileSystems."/" = {
    device = "/dev/disk/by-label/nixos";
    fsType = "ext4";
  };

  # Networking
  networking.hostName = "nixos";
  networking.networkmanager.enable = true;

  # Time zone
  time.timeZone = "UTC";

  # Internationalization
  i18n.defaultLocale = "en_US.UTF-8";

  # System packages - managed by slop
  environment.systemPackages = with pkgs; [
    vim
    wget
    git
  ];

  # Create a user (replace 'alice' with your username)
  users.users.alice = {
    isNormalUser = true;
    description = "Alice";
    extraGroups = [ "wheel" "networkmanager" ];
    shell = pkgs.bash;
  };

  # Enable sudo
  security.sudo.wheelNeedsPassword = true;

  # Nix settings
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nix.settings.auto-optimise-store = true;

  # State version - update to match your NixOS version
  system.stateVersion = "23.11";

}
