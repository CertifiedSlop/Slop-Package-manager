//! Slop - AI-powered package manager for NixOS
//!
//! Main application entry point and command handlers.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use slop::ai_interpreter::{ActionType, AiInterpreter};
use slop::cli::{Cli, Commands, FlakeCommands};
use slop::flake_manager::Flake;
use slop::nix_config::NixConfig;
use slop::package_resolver::PackageResolver;
use slop::rebuild::{is_nixos, RebuildExecutor};

/// Application state
pub struct App {
    config: NixConfig,
    resolver: PackageResolver,
    interpreter: AiInterpreter,
    executor: RebuildExecutor,
    dry_run: bool,
    verbose: bool,
    skip_confirm: bool,
}

impl App {
    /// Create a new application instance
    pub fn new(cli: &Cli) -> Result<Self> {
        let config_path = cli
            .config
            .clone()
            .unwrap_or_else(|| "/etc/nixos/configuration.nix".to_string());

        let config = NixConfig::load(&config_path)
            .with_context(|| format!("Failed to load config from {}", config_path))?;

        let resolver = PackageResolver::new();
        let interpreter = AiInterpreter::new(resolver.clone());
        let executor = RebuildExecutor::new(cli.dry_run, cli.verbose, !cli.yes);

        Ok(App {
            config,
            resolver,
            interpreter,
            executor,
            dry_run: cli.dry_run,
            verbose: cli.verbose,
            skip_confirm: cli.yes,
        })
    }

    /// Run the application
    pub async fn run(&mut self, cli: &Cli) -> Result<()> {
        match &cli.command {
            Commands::Install { package } => self.install(package).await,
            Commands::Remove { package } => self.remove(package).await,
            Commands::Search { query } => self.search(query),
            Commands::Ai { request } => self.ai(request).await,
            Commands::List => self.list(),
            Commands::Diff { add, remove } => {
                self.show_diff(add.as_ref(), remove.as_ref());
                Ok(())
            }
            Commands::Update { flake, input } => {
                if *flake {
                    self.update_flake(input.as_deref()).await
                } else {
                    self.update_packages().await
                }
            }
            Commands::Flake { action } => self.flake_command(action).await,
            Commands::Completions { shell } => Self::generate_completions(shell),
        }
    }

    /// Install a package
    async fn install(&mut self, package: &str) -> Result<()> {
        info!("Installing package: {}", package);

        // Resolve the package name
        let resolved = self
            .resolver
            .resolve(package)
            .context("Failed to resolve package name")?;

        if self.config.has_package(resolved) {
            println!(
                "{} Package '{}' is already installed",
                "ℹ".blue(),
                resolved.yellow()
            );
            return Ok(());
        }

        // Validate package exists (optional, may fail offline)
        if !self.resolver.validate_package(resolved).unwrap_or(true) {
            let suggestions = self.resolver.suggest(package);
            if !suggestions.is_empty() {
                println!("{}", "Did you mean:".yellow());
                for sug in suggestions {
                    println!("  - {}", sug.green());
                }
            }
            bail!("Package '{}' not found in nixpkgs", resolved);
        }

        // Backup current config
        let backup_path = self.config.backup().with_context(|| {
            if self.dry_run {
                "Would backup config".to_string()
            } else {
                "Failed to backup configuration".to_string()
            }
        })?;

        if self.verbose {
            println!("{} Backup created: {:?}", "ℹ".blue(), backup_path);
        }

        // Add package to config
        self.config.add_package(resolved)?;

        // Show diff
        let res_str = resolved.to_string();
        self.show_diff(None, Some(&res_str));

        // Confirm before applying
        if !self.skip_confirm
            && !self.dry_run
            && !self.executor.confirm("Apply changes and rebuild?")?
        {
            println!("{}", "Changes cancelled.".yellow());
            return Ok(());
        }

        if self.dry_run {
            println!(
                "{} Would add '{}' to environment.systemPackages",
                "→".yellow(),
                resolved.green()
            );
            println!("{} Would run: sudo nixos-rebuild switch", "→".yellow());
            return Ok(());
        }

        // Save configuration
        self.config.save().context("Failed to save configuration")?;

        println!("{} Configuration updated successfully", "✓".green());

        // Rebuild system
        let result = self.executor.rebuild()?;

        if !result.success {
            error!("Rebuild failed. Backup is at: {:?}", backup_path);
            bail!("System rebuild failed");
        }

        Ok(())
    }

    /// Remove a package
    async fn remove(&mut self, package: &str) -> Result<()> {
        info!("Removing package: {}", package);

        // Resolve the package name
        let resolved = self
            .resolver
            .resolve(package)
            .context("Failed to resolve package name")?;

        if !self.config.has_package(resolved) {
            println!(
                "{} Package '{}' is not installed",
                "ℹ".blue(),
                resolved.yellow()
            );
            return Ok(());
        }

        // Backup current config
        let backup_path = self.config.backup()?;

        if self.verbose {
            println!("{} Backup created: {:?}", "ℹ".blue(), backup_path);
        }

        // Remove package from config
        let removed = self.config.remove_package(resolved)?;

        if !removed {
            println!(
                "{} Package '{}' not found in configuration",
                "ℹ".blue(),
                resolved.yellow()
            );
            return Ok(());
        }

        // Show diff
        let res_str = resolved.to_string();
        self.show_diff(Some(&res_str), None);

        // Confirm before applying
        if !self.skip_confirm
            && !self.dry_run
            && !self.executor.confirm("Apply changes and rebuild?")?
        {
            println!("{}", "Changes cancelled.".yellow());
            return Ok(());
        }

        if self.dry_run {
            println!(
                "{} Would remove '{}' from environment.systemPackages",
                "→".yellow(),
                resolved.red()
            );
            println!("{} Would run: sudo nixos-rebuild switch", "→".yellow());
            return Ok(());
        }

        // Save configuration
        self.config.save().context("Failed to save configuration")?;

        println!("{} Configuration updated successfully", "✓".green());

        // Rebuild system
        let result = self.executor.rebuild()?;

        if !result.success {
            error!("Rebuild failed. Backup is at: {:?}", backup_path);
            bail!("System rebuild failed");
        }

        Ok(())
    }

    /// Search for packages
    fn search(&self, query: &str) -> Result<()> {
        info!("Searching for: {}", query);

        let results = self.resolver.search(query);

        if results.is_empty() {
            println!("{}", "No packages found.".yellow());
            return Ok(());
        }

        println!(
            "Found {} package(s):\n",
            results.len().to_string().green().bold()
        );

        for (i, result) in results.iter().take(10).enumerate() {
            println!(
                "{}. {} {}",
                (i + 1).to_string().cyan().bold(),
                result.attr_name.green(),
                result.package.description.as_deref().unwrap_or("").dimmed()
            );
        }

        if results.len() > 10 {
            println!("... and {} more", (results.len() - 10).to_string().dimmed());
        }

        Ok(())
    }

    /// Process AI request
    async fn ai(&mut self, request: &str) -> Result<()> {
        info!("Processing AI request: {}", request);

        let action = self.interpreter.interpret(request)?;

        println!(
            "{} Interpreted: {} (confidence: {:.0}%)",
            "🤖".blue(),
            format!("{:?}", action.action).green(),
            action.confidence * 100.0
        );

        if action.packages.is_empty() {
            println!("{}", "No packages identified.".yellow());
            return Ok(());
        }

        println!("Packages: {}", action.packages.join(", ").cyan());

        match action.action {
            ActionType::Install => {
                for package in &action.packages {
                    self.install(package).await?;
                }
            }
            ActionType::Remove => {
                for package in &action.packages {
                    self.remove(package).await?;
                }
            }
            ActionType::Search => {
                for package in &action.packages {
                    self.search(package)?;
                }
            }
            ActionType::Unknown => {
                println!("{}", "Could not determine action.".yellow());
            }
        }

        Ok(())
    }

    /// List installed packages
    fn list(&self) -> Result<()> {
        let packages = &self.config.packages;

        if packages.is_empty() {
            println!("{}", "No packages in environment.systemPackages".yellow());
            return Ok(());
        }

        println!(
            "{} package(s) installed:\n",
            packages.len().to_string().green().bold()
        );

        for package in packages {
            println!("  • {}", package.green());
        }

        Ok(())
    }

    /// Show diff of pending changes
    fn show_diff(&self, remove: Option<&String>, add: Option<&String>) {
        let old_packages = &self.config.packages;
        let mut new_packages = old_packages.clone();

        if let Some(pkg) = remove {
            new_packages.retain(|p| p != pkg);
        }

        if let Some(pkg) = add {
            if !new_packages.contains(pkg) {
                new_packages.push(pkg.clone());
            }
        }

        self.executor.show_diff(old_packages, &new_packages);
    }

    /// Update all packages (placeholder - full implementation would check for updates)
    async fn update_packages(&self) -> Result<()> {
        println!(
            "{} Checking for package updates...",
            "🔄".blue()
        );

        if self.dry_run {
            println!(
                "{} Would update all packages in environment.systemPackages",
                "→".yellow()
            );
            println!("{} Would run: sudo nixos-rebuild switch --upgrade", "→".yellow());
            return Ok(());
        }

        println!(
            "{} Note: Full package updates require running nixos-rebuild with --upgrade flag.",
            "ℹ".blue()
        );
        println!(
            "{} Run: sudo nixos-rebuild switch --upgrade",
            "💡".yellow()
        );

        Ok(())
    }

    /// Update flake inputs
    async fn update_flake(&self, input: Option<&str>) -> Result<()> {
        use slop::flake_manager::update_flake_inputs;

        let flake_path = Flake::default_path();

        if !Flake::exists(&flake_path) {
            bail!("No flake.nix found in current directory");
        }

        println!("{} Updating flake inputs...", "🔄".blue());

        if self.dry_run {
            if let Some(inp) = input {
                println!("{} Would update input: {}", "→".yellow(), inp);
            } else {
                println!("{} Would update all flake inputs", "→".yellow());
            }
            return Ok(());
        }

        let output = update_flake_inputs(&flake_path, input.is_none())?;

        if !output.is_empty() {
            println!("{}", output);
        }

        println!("{} Flake inputs updated successfully", "✓".green());

        Ok(())
    }

    /// Handle flake management commands
    async fn flake_command(&self, action: &FlakeCommands) -> Result<()> {
        let flake_path = Flake::default_path();

        match action {
            FlakeCommands::Add { name, url } => {
                if !Flake::exists(&flake_path) {
                    bail!("No flake.nix found in current directory");
                }

                let mut flake = Flake::load(&flake_path)?;

                if flake.has_input(name) {
                    println!(
                        "{} Input '{}' already exists",
                        "ℹ".blue(),
                        name.yellow()
                    );
                    return Ok(());
                }

                if self.dry_run {
                    println!(
                        "{} Would add input '{}' with URL '{}'",
                        "→".yellow(),
                        name,
                        url
                    );
                    return Ok(());
                }

                flake.add_input(name, url)?;
                flake.save()?;

                println!(
                    "{} Added flake input '{}' successfully",
                    "✓".green(),
                    name.green()
                );
            }
            FlakeCommands::Remove { name } => {
                if !Flake::exists(&flake_path) {
                    bail!("No flake.nix found in current directory");
                }

                let mut flake = Flake::load(&flake_path)?;

                if !flake.has_input(name) {
                    println!(
                        "{} Input '{}' not found",
                        "ℹ".blue(),
                        name.yellow()
                    );
                    return Ok(());
                }

                if self.dry_run {
                    println!("{} Would remove input: {}", "→".yellow(), name);
                    return Ok(());
                }

                flake.remove_input(name)?;
                flake.save()?;

                println!(
                    "{} Removed flake input '{}' successfully",
                    "✓".green(),
                    name.green()
                );
            }
            FlakeCommands::Update { name } => {
                if !Flake::exists(&flake_path) {
                    bail!("No flake.nix found in current directory");
                }

                if self.dry_run {
                    if let Some(inp) = name {
                        println!("{} Would update input: {}", "→".yellow(), inp);
                    } else {
                        println!("{} Would update all flake inputs", "→".yellow());
                    }
                    return Ok(());
                }

                use slop::flake_manager::update_flake_inputs;
                let output = update_flake_inputs(&flake_path, name.is_none())?;

                if !output.is_empty() {
                    println!("{}", output);
                }

                println!("{} Flake inputs updated successfully", "✓".green());
            }
            FlakeCommands::Lock => {
                if !Flake::exists(&flake_path) {
                    bail!("No flake.nix found in current directory");
                }

                if self.dry_run {
                    println!("{} Would lock flake inputs", "→".yellow());
                    return Ok(());
                }

                use slop::flake_manager::lock_flake_inputs;
                lock_flake_inputs(&flake_path)?;

                println!("{} Flake inputs locked successfully", "✓".green());
            }
            FlakeCommands::List => {
                if !Flake::exists(&flake_path) {
                    bail!("No flake.nix found in current directory");
                }

                let flake = Flake::load(&flake_path)?;

                if let Some(desc) = &flake.description {
                    println!("{} {}\n", "📦".blue(), desc.bold());
                }

                if flake.inputs.is_empty() {
                    println!("{}", "No flake inputs found.".yellow());
                } else {
                    println!("{} Flake inputs:\n", "📋".blue());
                    for input in &flake.inputs {
                        println!(
                            "  • {} {} {}",
                            input.name.green(),
                            "→".dimmed(),
                            input.url.dimmed()
                        );
                        if let Some(follows) = &input.follows {
                            println!("    {} follows: {}", "↳".dimmed(), follows.dimmed());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl App {
    /// Generate shell completions
    fn generate_completions(shell: &str) -> Result<()> {
        use clap::CommandFactory;
        use std::io;

        let mut cmd = Cli::command();
        let shell = shell.parse::<clap_complete::Shell>().unwrap_or(clap_complete::Shell::Bash);

        clap_complete::generate(
            shell,
            &mut cmd,
            "slop",
            &mut io::stdout(),
        );

        Ok(())
    }
}

/// Initialize logging
fn init_logging(verbose: bool) {
    let filter = if verbose {
        tracing_subscriber::filter::LevelFilter::DEBUG
    } else {
        tracing_subscriber::filter::LevelFilter::INFO
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_target(false),
        )
        .with(filter)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();

    init_logging(cli.verbose);

    // Check if running on NixOS (warning only, don't block)
    if !is_nixos() && !cli.dry_run {
        println!(
            "{} Warning: This doesn't appear to be a NixOS system.",
            "⚠".yellow()
        );
        println!(
            "{} Use --dry-run to test without making changes.",
            "ℹ".blue()
        );
    }

    let mut app = App::new(&cli)?;
    app.run(&cli).await?;

    Ok(())
}
