# Example configuration.nix for testing slop
# This is a minimal configuration for testing purposes

{ config, pkgs, ... }: {
  imports = [
    # Include the results of the hardware scan.
    ./hardware-configuration.nix
  ];

  # Bootloader.
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Define a user account.
  users.users.alice = {
    isNormalUser = true;
    description = "Alice";
    extraGroups = [ "networkmanager" "wheel" ];
    packages = with pkgs; [
      firefox
      thunderbird
    ];
  };

  # System packages - this is what slop modifies
  environment.systemPackages = with pkgs; [
    vim
    wget
    curl
    git
  ];

  # Network
  networking.hostName = "nixos";
  networking.networkmanager.enable = true;

  # Timezone
  time.timeZone = "UTC";

  # Locale
  i18n.defaultLocale = "en_US.UTF-8";

  # State version
  system.stateVersion = "23.11";
}
