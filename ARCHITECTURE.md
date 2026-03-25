# slop Architecture

> Technical architecture documentation for the slop AI-powered package manager

## Table of Contents

1. [Overview](#overview)
2. [Module Structure](#module-structure)
3. [Data Flow](#data-flow)
4. [AI Integration](#ai-integration)
5. [Safety Mechanisms](#safety-mechanisms)
6. [File Structure](#file-structure)

---

## Overview

slop is built with Rust and follows a modular architecture that separates concerns between:
- **CLI parsing** - Command-line interface handling
- **Configuration management** - Nix file parsing and editing
- **Package resolution** - Name resolution and search
- **AI processing** - Natural language interpretation
- **System operations** - Rebuild execution

### Design Principles

1. **Safe by default** - Backups, validation, confirmation prompts
2. **Offline-first** - Pattern matching works without API keys
3. **Extensible** - Modular design for easy feature addition
4. **Idiomatic Rust** - Following Rust API guidelines

---

## Module Structure

### Core Modules

```
src/
├── bin/
│   └── main.rs          # Entry point, CLI dispatch, App state
├── lib.rs               # Library root, re-exports all modules
├── cli.rs               # CLI argument definitions (clap)
├── nix_config.rs        # Configuration parsing and editing
├── package_resolver.rs  # Package name resolution
├── rebuild.rs           # System rebuild execution
├── flake_manager.rs     # Flake input management
└── ai_*.rs              # AI module suite
```

### Module Responsibilities

#### `cli.rs` - Command-Line Interface

Defines all commands using clap derive macros:

```rust
#[derive(Parser, Debug)]
pub struct Cli {
    pub verbose: bool,
    pub dry_run: bool,
    pub config: Option<String>,
    pub command: Commands,
}

pub enum Commands {
    Install { package: String },
    Remove { package: String },
    Search { query: String },
    Ai { request: String },
    // ... more commands
}
```

**Key features:**
- Global flags (--verbose, --dry-run, --yes, --config)
- Subcommand structure for all operations
- Shell completion generation

#### `nix_config.rs` - Configuration Parser/Editor

Handles parsing and modifying `configuration.nix`:

```rust
pub struct NixConfig {
    pub content: String,
    pub path: PathBuf,
    pub packages: Vec<String>,
    pub packages_range: Option<(usize, usize)>,
    pub uses_flakes: bool,
}
```

**Key methods:**
- `load(path)` - Parse configuration file
- `extract_packages()` - Regex-based package extraction
- `add_package()` / `remove_package()` - Modify packages list
- `validate()` - Syntax check with `nix-instantiate`
- `backup()` - Create timestamped backup
- `save()` - Atomic write with validation

**Regex patterns:**
```rust
// Match: environment.systemPackages = with pkgs; [ ... ];
r"(?s)environment\.systemPackages\s*=\s*(?:with\s+pkgs\s*;\s*)?\[([^\]]*)\]"
```

#### `package_resolver.rs` - Package Resolution

Handles package name resolution and search:

```rust
pub struct PackageResolver {
    package_aliases: HashMap<&'static str, &'static str>,
}
```

**Features:**
- 50+ built-in aliases (browser→firefox, nvim→neovim)
- Search via `nix-locate` integration
- Fuzzy suggestion for misspelled packages

**Alias categories:**
- Browsers (firefox, chrome, chromium, brave)
- Editors (neovim, vim, emacs, vscode)
- Terminals (alacritty, kitty, wezterm, foot)
- Shells (zsh, fish, bash, nushell)
- Dev tools (rustup, python3, nodejs, go)
- Utilities (htop, eza, ripgrep, fd, fzf)

#### `rebuild.rs` - Rebuild Executor

Handles system rebuild operations:

```rust
pub struct RebuildExecutor {
    dry_run: bool,
    verbose: bool,
    interactive: bool,
}
```

**Key methods:**
- `rebuild()` - Run `sudo nixos-rebuild switch`
- `show_diff()` - Display package changes
- `confirm()` - Interactive confirmation
- `check()` - Configuration validation

**Safety features:**
- Generation number extraction from output
- Colored output for success/failure
- Dry-run support for testing

#### `flake_manager.rs` - Flake Management

Handles flake.nix input management:

```rust
pub struct Flake {
    pub path: PathBuf,
    pub content: String,
    pub description: Option<String>,
    pub inputs: Vec<FlakeInput>,
    pub outputs: String,
}

pub struct FlakeInput {
    pub name: String,
    pub url: String,
    pub follows: Option<String>,
}
```

**Operations:**
- `load()` - Parse flake.nix
- `add_input()` - Add new input
- `remove_input()` - Remove input
- `update_input()` - Update input URL
- `save()` - Write changes

---

## AI Module Suite

### `ai_interpreter.rs` - Natural Language Processing

Core AI module that converts natural language to actions:

```rust
pub enum ActionType {
    Install,
    Remove,
    Search,
    Rollback,
    Optimize,
    Suggest,
    Unknown,
}

pub struct AiAction {
    pub action: ActionType,
    pub packages: Vec<String>,
    pub confidence: f32,
    pub original_request: String,
}
```

**Interpretation pipeline:**
1. Pattern matching (offline, fast)
2. LLM API fallback (if configured)
3. Default to search action

**LLM Providers:**
- OpenAI (via `SLOP_AI_API_KEY`)
- Ollama (local, via `SLOP_OLLAMA_URL`)
- OpenRouter (via `SLOP_OPENROUTER_KEY`)

### `ai_context.rs` - System Context

Maintains awareness of system configuration:

```rust
pub struct AiContext {
    config_path: PathBuf,
    installed_packages: Vec<String>,
    detected_categories: Vec<String>,
}
```

**Features:**
- Detects installed packages
- Identifies development languages
- Categorizes system use (desktop, server, dev)

### `ai_memory.rs` - Conversation History

Manages conversation state and preferences:

```rust
pub struct AiMemory {
    history_path: PathBuf,
    messages: Vec<Message>,
    preferences: UserPreferences,
}

pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub packages: Vec<String>,
    pub timestamp: u64,
}
```

**Features:**
- Persistent conversation history
- Package installation tracking
- Reference resolution ("install it" → resolves to previous package)

### `ai_bundles.rs` - Package Bundles

Pre-configured package sets for common use cases:

```rust
pub struct Bundle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
    pub optional_packages: Vec<String>,
}
```

**Built-in bundles:**
- `dev-rust` - Rust development toolchain
- `dev-python` - Python development
- `dev-web` - Web development
- `utils-cli` - Essential CLI tools
- `desktop-essentials` - Desktop utilities
- `gaming` - Gaming-related packages

### `ai_search.rs` - Semantic Search

Semantic package search engine:

```rust
pub struct SemanticSearchEngine {
    resolver: PackageResolver,
    use_case_map: HashMap<&'static str, Vec<&'static str>>,
}

pub enum MatchType {
    Exact,
    Semantic,
    Category,
    Related,
    Fuzzy,
}
```

**Match types:**
- Exact - Direct package name match
- Semantic - Use case matching ("video editing" → kdenlive)
- Category - Category-based (browsers, editors)
- Related - Associated packages
- Fuzzy - Levenshtein distance matching

### `ai_wizard.rs` - Setup Wizard

Interactive guided setup:

```rust
pub struct SetupWizard {
    verbose: bool,
    preferences: UserPreferences,
}
```

**Wizard flow:**
1. Welcome and introduction
2. Use case selection
3. Tool preferences (editor, terminal, shell)
4. Development languages
5. Desktop environment
6. Generate recommendations

### `ai_optimizer.rs` - Configuration Optimizer

Analyzes configuration for improvements:

```rust
pub struct ConfigOptimizer {
    config_path: PathBuf,
    config: NixConfig,
}

pub struct OptimizationReport {
    pub total_suggestions: usize,
    pub suggestions: Vec<OptimizationSuggestion>,
}
```

**Optimization categories:**
- Unused package detection
- Redundant module detection
- Performance improvements
- Size reduction suggestions

### `ai_hardware.rs` - Hardware Detection

Detects hardware and recommends drivers:

```rust
pub struct HardwareDetector {
    gpu: Option<GpuInfo>,
    network: Option<NetworkInfo>,
    audio: Option<AudioInfo>,
}
```

**Detection methods:**
- `/sys` filesystem parsing
- `lspci` / `lsusb` output parsing
- NixOS module recommendations

### `ai_conflicts.rs` - Conflict Detection

Detects package conflicts:

```rust
pub struct ConflictDetector {
    installed_packages: Vec<String>,
}

pub struct ConflictReport {
    pub total: usize,
    pub conflicts: Vec<Conflict>,
}
```

**Conflict types:**
- Multiple browsers (firefox + chromium + brave)
- Multiple editors (vim + neovim + emacs)
- Incompatible services

### `ai_health.rs` - Health Check

System health monitoring:

```rust
pub struct HealthChecker {
    config_path: PathBuf,
}

pub struct HealthReport {
    pub disk_usage: DiskStatus,
    pub config_status: ConfigStatus,
    pub security_status: SecurityStatus,
}
```

**Health checks:**
- Disk usage analysis
- Configuration syntax validation
- Service status checks
- Security update availability

---

## Data Flow

### Install Operation Flow

```
┌─────────────┐
│   User      │
│ "slop install│
│  firefox"   │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│  CLI Parser (cli.rs)            │
│  - Parse arguments              │
│  - Validate flags               │
│  - Dispatch to handler          │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  App::install() (main.rs)       │
│  - Resolve package name         │
│  - Check if already installed   │
│  - Validate package exists      │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  NixConfig::backup()            │
│  - Create timestamped backup    │
│  - Store in same directory      │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  NixConfig::add_package()       │
│  - Add to packages Vec          │
│  - Rebuild content string       │
│  - Update packages_range        │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  RebuildExecutor::show_diff()   │
│  - Compare old vs new packages  │
│  - Display added/removed        │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  User Confirmation              │
│  - Interactive prompt           │
│  - Skip if --yes flag           │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  NixConfig::validate()          │
│  - Write to temp file           │
│  - Run nix-instantiate --parse  │
│  - Check exit status            │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  NixConfig::save()              │
│  - Atomic write to disk         │
│  - Replace original file        │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  RebuildExecutor::rebuild()     │
│  - Run sudo nixos-rebuild switch│
│  - Capture output               │
│  - Extract generation number    │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────┐
│   Success   │
│  or Error   │
└─────────────┘
```

### AI Request Flow

```
┌─────────────┐
│   User      │
│ "I need a   │
│  browser"   │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│  AiInterpreter::interpret()     │
│  - Convert to lowercase         │
│  - Check for special patterns   │
└──────────────┬──────────────────┘
               │
       ┌───────┴───────┐
       │               │
       ▼               ▼
┌─────────────┐ ┌─────────────┐
│  Pattern    │ │  Rollback/  │
│  Matching   │ │  Optimize   │
│  (offline)  │ │  detected   │
└──────┬──────┘ └──────┬──────┘
       │               │
       │    ┌──────────┘
       │    │
       ▼    ▼
┌─────────────────────────────────┐
│  Action Determination           │
│  - Install/Remove/Search        │
│  - Package resolution           │
│  - Confidence scoring           │
└──────────────┬──────────────────┘
               │
       ┌───────┴───────┐
       │               │
       ▼               ▼
┌─────────────┐ ┌─────────────┐
│  Confident  │ │  Low        │
│  (≥0.7)     │ │  Confidence │
└──────┬──────┘ └──────┬──────┘
       │               │
       │               ▼
       │      ┌────────────────┐
       │      │  LLM Fallback  │
       │      │  (if configured)│
       │      └───────┬────────┘
       │              │
       ▼              ▼
┌─────────────────────────────────┐
│  AiAction Result                │
│  - action: ActionType           │
│  - packages: Vec<String>        │
│  - confidence: f32              │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  App::ai() Handler              │
│  - Execute action               │
│  - Call install/remove/search   │
└─────────────────────────────────┘
```

---

## AI Integration

### Provider Configuration

```rust
pub enum LlmProvider {
    OpenAI,
    Ollama { url: String },
    OpenRouter,
}
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `SLOP_AI_API_KEY` | OpenAI API key | - |
| `SLOP_AI_API_URL` | Custom API endpoint | OpenAI default |
| `SLOP_OLLAMA_URL` | Ollama server URL | - |
| `SLOP_OLLAMA_MODEL` | Ollama model | `llama3.2` |
| `SLOP_OPENROUTER_KEY` | OpenRouter API key | - |
| `SLOP_OPENROUTER_MODEL` | OpenRouter model | `llama-3.2-3b-instruct:free` |

### LLM Request Format

```json
{
  "model": "gpt-3.5-turbo",
  "messages": [
    {
      "role": "system",
      "content": "You are a NixOS package management assistant..."
    },
    {
      "role": "user",
      "content": "Request: install a browser"
    }
  ],
  "temperature": 0.3,
  "max_tokens": 150
}
```

### Expected Response

```json
{
  "action": "install",
  "packages": ["firefox", "chromium"],
  "confidence": 0.9
}
```

---

## Safety Mechanisms

### Backup System

```rust
pub fn backup(&self) -> Result<PathBuf> {
    let backup_path = self.path.with_extension(format!(
        "nix.bak.{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    ));
    copy(&self.path, &backup_path, &CopyOptions::new())?;
    Ok(backup_path)
}
```

**Features:**
- Timestamped backups (`.nix.bak.1234567890`)
- Same directory as original
- Preserves original permissions

### Validation Pipeline

```
1. Pre-edit validation
   └─→ Package exists in nixpkgs

2. Post-edit validation
   └─→ nix-instantiate --parse <config>

3. Pre-rebuild validation
   └─→ nix-instantiate --check <config>
```

### Confirmation Flow

```
┌─────────────────────────────────────┐
│  Changes detected:                  │
│    + firefox                        │
│    + neovim                         │
│                                     │
│  Apply changes and rebuild? [y/N]:  │
└─────────────────────────────────────┘
```

**Skip conditions:**
- `--yes` flag provided
- Non-interactive mode

### Dry-Run Mode

```rust
pub fn is_dry_run(&self) -> bool {
    self.dry_run
}
```

**Behavior:**
- Creates backup (logs path only)
- Shows diff without saving
- Shows rebuild command without executing
- Returns success immediately

---

## File Structure

```
slop/
├── .cargo/
│   └── config.toml          # Cargo configuration
├── .devcontainer/
│   └── devcontainer.json    # Dev container config
├── .github/
│   └── workflows/
│       ├── ci.yml           # CI/CD pipeline
│       └── release.yml      # Release automation
├── examples/
│   └── configuration.nix    # Example config
├── src/
│   ├── bin/
│   │   └── main.rs          # Entry point
│   ├── lib.rs               # Library root
│   ├── cli.rs               # CLI definitions
│   ├── nix_config.rs        # Config parser
│   ├── package_resolver.rs  # Package resolution
│   ├── rebuild.rs           # Rebuild executor
│   ├── flake_manager.rs     # Flake management
│   ├── ai_interpreter.rs    # AI core
│   ├── ai_context.rs        # Context awareness
│   ├── ai_memory.rs         # Conversation history
│   ├── ai_bundles.rs        # Package bundles
│   ├── ai_search.rs         # Semantic search
│   ├── ai_wizard.rs         # Setup wizard
│   ├── ai_optimizer.rs      # Config optimizer
│   ├── ai_hardware.rs       # Hardware detection
│   ├── ai_conflicts.rs      # Conflict detection
│   └── ai_health.rs         # Health checks
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── integration_tests.rs # Additional tests
├── Cargo.toml               # Dependencies
├── Cargo.lock               # Locked dependencies
├── flake.nix                # Nix flake
├── flake.lock               # Locked flake inputs
├── rust-toolchain.toml      # Rust version
├── rustfmt.toml             # Formatting config
├── shell.nix                # Nix shell
├── README.md                # Main documentation
├── USAGE.md                 # Usage guide
├── CONTRIBUTING.md          # Contribution guide
├── SECURITY.md              # Security policy
├── CHANGELOG.md             # Version history
└── CODE_REVIEW.md           # Code review guidelines
```

---

## Testing Strategy

### Unit Tests

Each module has comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_packages() { ... }

    #[test]
    fn test_add_package() { ... }

    #[test]
    fn test_remove_package() { ... }
}
```

### Integration Tests

End-to-end tests in `tests/`:

```rust
#[test]
fn test_full_install_workflow() { ... }

#[test]
fn test_full_ai_workflow() { ... }

#[test]
fn test_config_load_and_modify() { ... }
```

### Test Coverage

- **56 unit tests** - Module-level functionality
- **17 integration tests** - End-to-end workflows
- **29 additional tests** - Extended coverage
- **102 total tests**

---

<div align="center">

**For implementation details, see the source code in `src/`**

</div>
