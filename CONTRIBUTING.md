# Contributing to slop

Thank you for considering contributing to slop! We welcome contributions from the community.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/slop.git
   cd slop
   ```
3. **Add upstream** remote:
   ```bash
   git remote add upstream https://github.com/yourusername/slop.git
   ```

## Development Setup

### Using Dev Container (Recommended)

The easiest way to get started is using the included dev container:

1. Install [VS Code](https://code.visualstudio.com/)
2. Install the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
3. Open the project in VS Code
4. Click "Reopen in Container" when prompted

### Manual Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies (Ubuntu/Debian)
sudo apt-get install -y pkg-config libssl-dev

# Clone the repository
git clone https://github.com/YOUR_USERNAME/slop.git
cd slop

# Build the project
cargo build

# Run tests
cargo test
```

## Making Changes

### Finding Issues to Work On

- Look for issues labeled `good first issue` for beginner-friendly tasks
- Issues labeled `help wanted` need community contributions
- Check the [project board](https://github.com/yourusername/slop/projects) for ongoing work

### Creating a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create a feature branch
git checkout -b feature/your-feature-name
```

### Branch Naming Convention

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions/modifications
- `chore/` - Maintenance tasks

## Pull Request Guidelines

### Before Submitting

1. **Run tests**: `cargo test`
2. **Run clippy**: `cargo clippy -- -D warnings`
3. **Format code**: `cargo fmt`
4. **Update documentation** if needed
5. **Add tests** for new functionality

### PR Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass
- [ ] New tests added (if applicable)

## Checklist
- [ ] Code follows project guidelines
- [ ] Self-review completed
- [ ] Comments added where necessary
- [ ] Documentation updated
```

## Coding Standards

### Rust Style Guide

We follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// Use descriptive variable names
let package_name = "firefox";

// Handle errors properly
fn load_config(path: &str) -> Result<NixConfig> {
    // ...
}

// Add documentation for public items
/// Parse a Nix configuration file
/// 
/// # Arguments
/// * `path` - Path to the configuration file
pub fn parse(path: &str) -> Result<Self> {
    // ...
}
```

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add AI-powered package search
fix: prevent duplicate packages in config
docs: update installation instructions
refactor: simplify config parsing logic
test: add unit tests for package resolver
chore: update dependencies
```

### Code Organization

```
src/
├── main.rs           # Entry point and CLI handling
├── cli.rs            # Command-line interface definitions
├── nix_config.rs     # Nix configuration parsing/editing
├── package_resolver.rs  # Package name resolution
├── ai_interpreter.rs    # Natural language processing
└── rebuild.rs        # System rebuild execution
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test nix_config

# Integration tests
cargo test --test integration
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_package() {
        let mut config = NixConfig::new();
        config.add_package("firefox").unwrap();
        assert!(config.has_package("firefox"));
    }

    #[test]
    fn test_remove_nonexistent_package() {
        let mut config = NixConfig::new();
        let result = config.remove_package("nonexistent");
        assert!(!result.unwrap());
    }
}
```

## Documentation

### Code Comments

- Add doc comments (`///`) for public items
- Explain *why*, not just *what*
- Include examples for complex functions

### README Updates

Update the README when:
- Adding new commands
- Changing existing behavior
- Adding configuration options

### CHANGELOG

Add entries to `CHANGELOG.md` for user-facing changes:

```markdown
## [Unreleased]

### Added
- New AI command for natural language package requests

### Changed
- Improved package resolution with fuzzy matching

### Fixed
- Backup file creation on Windows
```

## Release Process

Releases are managed by maintainers:

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml`
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions creates release

## Getting Help

- **Discussions**: [GitHub Discussions](https://github.com/yourusername/slop/discussions)
- **Issues**: [GitHub Issues](https://github.com/yourusername/slop/issues)
- **Matrix/Chat**: (if applicable)

## Thank You!

Every contribution makes slop better. Whether it's a bug fix, feature, or documentation improvement - we appreciate it! 🦀

---

<div align="center">

**Happy Contributing!**

</div>
