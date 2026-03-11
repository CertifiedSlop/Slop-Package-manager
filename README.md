# 🦥 slop

> **AI-Powered Package Manager for NixOS**

<div align="center">

[![CI](https://github.com/yourusername/slop/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/slop/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://rust-lang.org)
[![NixOS](https://img.shields.io/badge/NixOS-unstable-blue.svg)](https://nixos.org)

![Demo](https://img.shields.io/badge/Status-Experimental-orange)
![Crates.io](https://img.shields.io/crates/v/slop)

</div>

---

## 🎯 What is slop?

`slop` is an experimental CLI tool that brings **AI-assisted package management** to NixOS. Instead of manually editing `/etc/nixos/configuration.nix`, you can simply run:

```bash
slop install firefox
slop ai "i need a browser"
```

And slop will automatically:
1. Parse your configuration
2. Add the package to `environment.systemPackages`
3. Validate the syntax
4. Run `nixos-rebuild switch`

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🚀 **Quick Install** | `slop install <package>` - done! |
| 🤖 **AI Commands** | Natural language: `"install a terminal editor"` |
| 🛡️ **Safe by Default** | Backups, validation, dry-run mode |
| 📦 **Smart Resolution** | 50+ built-in aliases (nvim→neovim, browser→firefox) |
| 🎨 **Beautiful UX** | Colored output, diffs, interactive prompts |
| 🔍 **Search** | Find packages with `slop search <query>` |

## 📦 Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/slop.git
cd slop

# Build in release mode
cargo build --release

# Install to your PATH
sudo cp target/release/slop /usr/local/bin/
```

### Option 2: Using Nix (Flakes)

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    slop.url = "github:yourusername/slop";
  };

  outputs = { self, nixpkgs, slop }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [
            slop.packages.${pkgs.system}.default
          ];
        })
      ];
    };
  };
}
```

### Option 3: Dev Container

```bash
# Open in VS Code with Dev Containers extension
# Or run:
devcontainer up --workspace-folder .
```

## 📖 Usage

### Basic Commands

```bash
# Install packages
slop install firefox
slop install neovim git htop

# Remove packages
slop remove firefox

# Search for packages
slop search editor
slop search terminal

# List installed packages
slop list

# Preview changes
slop --dry-run install vscode
```

### AI-Powered Commands

```bash
# Natural language installation
slop ai "install a web browser"
slop ai "i need a terminal editor"
slop ai "remove the web browser"
slop ai "set me up for rust development"

# AI will interpret and execute the appropriate actions
```

### Global Options

```bash
slop --help
slop --verbose install firefox      # Verbose output
slop --dry-run remove neovim        # Preview without applying
slop --yes install git              # Skip confirmation prompts
slop --config /path/to/config.nix   # Custom config path
```

## 🎯 Package Aliases

Slop includes smart aliases for common packages:

| Category | Aliases | Resolves To |
|----------|---------|-------------|
| **Browsers** | `browser`, `firefox` | `firefox` |
| | `chrome`, `google-chrome` | `google-chrome` |
| | `chromium`, `brave` | `chromium`, `brave` |
| **Editors** | `editor`, `nvim`, `neovim` | `neovim` |
| | `vim`, `emacs`, `vscode`, `code` | `vim`, `emacs`, `vscode` |
| **Terminals** | `terminal`, `term`, `alacritty` | `alacritty` |
| | `kitty`, `wezterm`, `foot` | `kitty`, `wezterm`, `foot` |
| **Shells** | `shell`, `zsh`, `fish`, `bash` | `zsh`, `fish`, `bash` |
| **Dev Tools** | `rust`, `cargo`, `rustc` | `rustup` |
| | `python`, `python3` | `python3` |
| | `node`, `nodejs` | `nodejs` |
| | `go`, `golang` | `go` |
| **Utilities** | `top`, `htop`, `btop` | `htop`, `btop` |
| | `ls`, `eza` | `eza` |
| | `rg`, `ripgrep` | `ripgrep` |
| | `fd`, `fzf`, `bat` | `fd`, `fzf`, `bat` |

## 🛡️ Safety Features

```
┌─────────────────────────────────────────────────────────┐
│  Before Making Changes:                                 │
│  1. ✅ Create timestamped backup of configuration.nix   │
│  2. ✅ Validate Nix syntax with nix-instantiate         │
│  3. ✅ Show diff of pending changes                     │
│  4. ✅ Ask for user confirmation                        │
│  5. ✅ Only then run nixos-rebuild switch               │
└─────────────────────────────────────────────────────────┘
```

### Dry-Run Mode

Always test before applying:

```bash
$ slop --dry-run install firefox

[DRY RUN] Would backup config
[DRY RUN] Would add 'firefox' to environment.systemPackages
[DRY RUN] Would run: sudo nixos-rebuild switch
```

## 📋 Command Reference

| Command | Aliases | Description |
|---------|---------|-------------|
| `slop install <pkg>` | `i`, `add` | Install a package |
| `slop remove <pkg>` | `rm`, `delete` | Remove a package |
| `slop search <query>` | `find`, `lookup` | Search packages |
| `slop ai "<request>"` | `ask` | Natural language |
| `slop list` | `ls`, `installed` | List packages |
| `slop diff` | `preview` | Show pending changes |

## 🔧 Configuration

### AI API Integration (Optional)

Slop supports multiple LLM providers for AI commands. Without any configuration, slop uses pattern matching (works offline!).

#### OpenAI

```bash
export SLOP_AI_API_KEY="sk-..."
export SLOP_AI_API_URL="https://api.openai.com/v1/chat/completions"
# Or simply:
export OPENAI_API_KEY="sk-..."
```

#### Ollama (Local LLM - Free, Private)

```bash
# Install Ollama: https://ollama.ai
ollama pull llama3.2

# Configure slop to use Ollama
export SLOP_OLLAMA_URL="http://localhost:11434"
export SLOP_OLLAMA_MODEL="llama3.2"
```

#### OpenRouter (Multiple Models, Pay-per-use)

```bash
# Get API key: https://openrouter.ai
export SLOP_OPENROUTER_KEY="sk-or-..."
export SLOP_OPENROUTER_MODEL="meta-llama/llama-3.2-3b-instruct:free"
# Or:
export OPENROUTER_API_KEY="sk-or-..."
```

### Provider Comparison

| Provider | Cost | Privacy | Speed | Quality |
|----------|------|---------|-------|---------|
| Pattern Matching | Free | ✅ Local | ⚡ Instant | Basic |
| Ollama (local) | Free | ✅ Local | 🐛 Depends on GPU | Good |
| OpenRouter | $/req | ⚠️ Cloud | ⚡ Fast | Excellent |
| OpenAI | $/req | ⚠️ Cloud | ⚡ Fast | Excellent |

### Example configuration.nix

**Before:**
```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  environment.systemPackages = with pkgs; [
    vim
    wget
    git
  ];
}
```

**After running `slop install firefox neovim`:**
```nix
{ config, pkgs, ... }: {
  imports = [ ./hardware-configuration.nix ];

  environment.systemPackages = with pkgs; [
    vim wget git firefox neovim
  ];
}
```

## 🏗️ Architecture

```
┌────────────────────────────────────────────────────────────┐
│                      CLI (clap)                            │
│   install │ remove │ search │ ai │ list │ diff             │
└────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│                    Application Core                        │
├───────────────┬───────────────┬───────────────┬────────────┤
│  NixConfig    │  Package      │  AI           │  Rebuild   │
│  Parser       │  Resolver     │  Interpreter  │  Executor  │
│               │               │               │            │
│  • Parse      │  • Resolve    │  • Pattern    │  • Backup  │
│  • Edit       │  • Search     │  • LLM API    │  • Validate│
│  • Validate   │  • Suggest    │  • Actions    │  • Rebuild │
└───────────────┴───────────────┴───────────────┴────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│                   NixOS System                             │
│         /etc/nixos/configuration.nix + nixos-rebuild       │
└────────────────────────────────────────────────────────────┘
```

## 🧪 Development

### Prerequisites

- Rust 1.70+
- Nix (optional, for NixOS features)
- OpenSSL development libraries

### Quick Start

```bash
# Clone and enter the project
git clone https://github.com/yourusername/slop.git
cd slop

# Build
cargo build

# Run tests
cargo test

# Run with verbose output
cargo run -- --verbose install firefox

# Use dev container (VS Code)
# Install "Dev Containers" extension and reopen in container
```

### Project Structure

```
slop/
├── .devcontainer/       # Dev container config
├── .github/workflows/   # CI/CD pipelines
├── examples/            # Example configurations
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI parsing
│   ├── nix_config.rs    # Config parser
│   ├── package_resolver.rs  # Package resolution
│   ├── ai_interpreter.rs    # AI processing
│   └── rebuild.rs       # Rebuild execution
├── Cargo.toml           # Dependencies
├── flake.nix            # Nix flake
└── README.md            # This file
```

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test nix_config

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Linting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| `Failed to read config` | Ensure `/etc/nixos/configuration.nix` exists or use `--config` |
| `nix-instantiate: command not found` | Install nix: `nix-env -iA nixpkgs.nix` |
| `Permission denied` | Run with sudo or ensure write access |
| `Rebuild failed` | Check error output, use `--verbose` for details |
| `Not on NixOS` | Use `--dry-run` for testing on other systems |

## 🔒 Security Considerations

1. **Sudo Required**: `nixos-rebuild switch` needs elevated privileges
2. **Backups**: Stored alongside original config with timestamps
3. **API Keys**: Use environment variables, never commit to config
4. **Validation**: Syntax checked before any changes applied
5. **Audit**: Run `cargo audit` to check for vulnerable dependencies

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/slop.git

# Create a branch
git checkout -b feature/my-feature

# Make changes and test
cargo test

# Commit and push
git commit -m "feat: add my feature"
git push origin feature/my-feature
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [NixOS](https://nixos.org) - The purely functional Linux distribution
- [clap](https://github.com/clap-rs/clap) - CLI argument parser
- [similar](https://github.com/mitsuhiko/similar) - Diff library
- [tokio](https://tokio.rs) - Async runtime
- [dialoguer](https://github.com/console-rs/dialoguer) - Interactive prompts

## 📮 Contact

- **Issues**: [GitHub Issues](https://github.com/yourusername/slop/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/slop/discussions)

---

<div align="center">

**Made with ❤️ and 🦀 for the NixOS community**

*⚠️ This is experimental software. Always review changes and keep backups!*

```
 _   _  ___  __ _  ___ 
| \ | |/ _ \/ _` |/ __|
|  \| | | | | (_| | (__ 
| |\  | | | |\__,_|\___|
|_| \_|_| |_|  |_| |_| 
```

</div>
