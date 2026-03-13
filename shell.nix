# Development shell for slop
# Usage: nix-shell shell.nix

{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball {
      url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
    }))
  ];
} }:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rustfmt" "clippy" "rust-src" ];
  };
in
pkgs.mkShell {
  name = "slop-dev";

  buildInputs = with pkgs; [
    # Rust toolchain
    rustToolchain

    # OpenSSL for reqwest
    openssl
    pkg-config

    # Cargo utilities
    cargo-edit
    cargo-watch

    # Optional: nix tools for testing
    nix
    nixpkgs-fmt
  ];

  shellHook = ''
    export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
    echo "🦥 slop development environment loaded!"
    echo ""
    echo "Available commands:"
    echo "  cargo build     - Build the project"
    echo "  cargo run       - Run the project"
    echo "  cargo test      - Run tests"
    echo "  cargo clippy    - Run linter"
    echo "  cargo watch     - Watch for changes and rebuild"
    echo ""
  '';
}
