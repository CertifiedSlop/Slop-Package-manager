FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install Nix
RUN apt-get update && apt-get install -y \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Nix
RUN sh <(curl -L https://nixos.org/nix/install) --no-daemon

# Configure Nix for flakes
RUN echo "experimental-features = nix-command flakes" >> /home/vscode/.config/nix/nix.conf

# Add Nix to PATH
ENV PATH="/home/vscode/.nix-profile/bin:${PATH}"

# Install additional development tools
RUN su vscode -c "cargo install cargo-edit cargo-watch cargo-audit"
