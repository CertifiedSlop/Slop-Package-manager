[flake]
name = "slop"
description = "AI-powered package manager for NixOS"
inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
inputs.flake-utils.url = "github:numtide/flake-utils";

[outputs]
{ self, nixpkgs, flake-utils }:
flake-utils.lib.eachDefaultSystem (system:
let
  pkgs = nixpkgs.legacyPackages.${system};
in
{
  packages.default = pkgs.rustPlatform.buildRustPackage {
    pname = "slop";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    
    nativeBuildInputs = with pkgs; [
      pkg-config
      rustPlatform.bindgenHook
    ];
    
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
    
    OPENSSL_NO_VENDOR = "1";
    
    meta = with pkgs.lib; {
      description = "AI-powered package manager for NixOS";
      homepage = "https://github.com/yourusername/slop";
      license = licenses.mit;
      maintainers = [ maintainers.yourusername ];
      mainProgram = "slop";
    };
  };

  devShells.default = pkgs.mkShell {
    buildInputs = with pkgs; [
      rustc
      cargo
      rustfmt
      clippy
      openssl
      pkg-config
    ];

    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };
})
