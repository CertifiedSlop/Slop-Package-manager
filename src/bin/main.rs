//! Slop - AI-powered package manager for NixOS
//!
//! Main application entry point and command handlers.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use slop::ai_bundles::BundleManager;
use slop::ai_conflicts::ConflictDetector;
use slop::ai_context::AiContext;
use slop::ai_hardware::HardwareDetector;
use slop::ai_health::HealthChecker;
use slop::ai_interpreter::{ActionType, AiAction, AiInterpreter};
use slop::ai_memory::AiMemory;
use slop::ai_optimizer::ConfigOptimizer;
use slop::ai_search::SemanticSearchEngine;
use slop::ai_wizard::SetupWizard;
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
            Commands::Search { query, semantic } => {
                if *semantic {
                    self.semantic_search(query)
                } else {
                    self.search(query)
                }
            }
            Commands::Ai { request } => self.ai(request).await,
            Commands::AiSuggest { category } => self.ai_suggest(category.as_deref()),
            Commands::AiOptimize { dry_run } => self.ai_optimize(*dry_run),
            Commands::AiChat => self.ai_chat().await,
            Commands::AiSetup => self.ai_setup(),
            Commands::AiDetectHardware => self.ai_detect_hardware(),
            Commands::AiCheckConflicts { packages } => self.ai_check_conflicts(packages),
            Commands::AiHealth => self.ai_health(),
            Commands::AiHistory { limit } => self.ai_history(*limit),
            Commands::AiClearHistory => self.ai_clear_history(),
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

    /// Get AI-powered package suggestions
    fn ai_suggest(&self, category: Option<&str>) -> Result<()> {
        println!("{} Getting AI-powered suggestions...", "🤖".blue());

        // Get context
        let context = match AiContext::new("/etc/nixos/configuration.nix") {
            Ok(ctx) => ctx,
            Err(_) => {
                println!("{}", "⚠ Could not load configuration context".yellow());
                AiContext::default_context()
            }
        };

        // Get bundle manager
        let bundle_manager = BundleManager::new();

        if let Some(cat) = category {
            // Search for bundles matching the category
            let bundles = bundle_manager.search_bundles(cat);

            if bundles.is_empty() {
                // Try to suggest based on context
                let suggestions = context.suggest_packages(&self.resolver, cat);

                if suggestions.is_empty() {
                    println!("{}", "No specific suggestions for this category.".yellow());
                    println!("\nAvailable bundle categories:");
                    for cat in bundle_manager.get_categories() {
                        println!("  • {}", cat.green());
                    }
                } else {
                    println!("\n{} Based on your configuration:\n", "💡".green());
                    for (pkg, reason) in suggestions {
                        println!("  {} {} - {}", "→".cyan(), pkg.green(), reason.dimmed());
                    }
                }
            } else {
                println!("\n{} Found {} bundle(s):\n", "📦".green(), bundles.len());
                for bundle in bundles {
                    println!(
                        "{} {} ({})",
                        "📦".blue(),
                        bundle.name.bold(),
                        bundle.id.dimmed()
                    );
                    println!("   {}", bundle.description);
                    println!("   Packages: {}", bundle.packages.join(", ").green());
                    if !bundle.optional_packages.is_empty() {
                        println!(
                            "   Optional: {}",
                            bundle.optional_packages.join(", ").yellow()
                        );
                    }
                    println!();
                }
            }
        } else {
            // Show all available categories and popular bundles
            println!("\n{} Available bundle categories:\n", "📂".green());
            for cat in bundle_manager.get_categories() {
                let count = bundle_manager.get_bundles_by_category(&cat).len();
                println!("  {} {} ({} bundles)", "📂".blue(), cat.green(), count);
            }

            println!("\n{} Popular bundles:\n", "⭐".green());
            let popular = [
                "dev-rust",
                "dev-python",
                "dev-web",
                "utils-cli",
                "desktop-essentials",
            ];
            for id in &popular {
                if let Some(bundle) = bundle_manager.get_bundle(id) {
                    println!(
                        "  {} {} - {}",
                        "⭐".yellow(),
                        bundle.name.bold(),
                        bundle.description.dimmed()
                    );
                }
            }

            println!("\n{} Usage: slop ai-suggest <category>", "💡".blue());
            println!("   Examples: slop ai-suggest rust, slop ai-suggest gaming");
        }

        Ok(())
    }

    /// Optimize configuration with AI
    fn ai_optimize(&self, dry_run: bool) -> Result<()> {
        println!(
            "{} Analyzing configuration for optimizations...",
            "🤖".blue()
        );

        let optimizer = match ConfigOptimizer::new("/etc/nixos/configuration.nix") {
            Ok(opt) => opt,
            Err(e) => {
                println!("{} Could not load configuration: {}", "⚠".yellow(), e);
                return Ok(());
            }
        };

        let report = optimizer.generate_report();

        if report.total_suggestions == 0 {
            println!("\n{} Your configuration looks well optimized!", "✓".green());
            return Ok(());
        }

        println!("\n{}", report.summary().bold());
        println!();

        for (i, suggestion) in report.suggestions.iter().enumerate() {
            let impact_icon = match suggestion.impact {
                slop::ai_optimizer::ImpactLevel::High => "🔴",
                slop::ai_optimizer::ImpactLevel::Medium => "🟡",
                slop::ai_optimizer::ImpactLevel::Low => "🟢",
            };

            println!(
                "{}. {} {}",
                (i + 1).to_string().cyan().bold(),
                impact_icon,
                suggestion.title.bold()
            );
            println!("   {}", suggestion.description);

            if let Some(savings) = &suggestion.estimated_savings {
                println!("   {} Potential: {}", "💰".green(), savings.green());
            }
            println!();
        }

        if !dry_run {
            println!(
                "{} To apply optimizations, review each suggestion manually.",
                "ℹ".blue()
            );
            println!("   Some optimizations may require manual intervention.");
        } else {
            println!("{} Dry run mode - no changes made.", "ℹ".blue());
        }

        Ok(())
    }

    /// Interactive AI chat mode
    async fn ai_chat(&mut self) -> Result<()> {
        use std::io::{self, BufRead, Write};

        println!(
            "{} Entering AI chat mode. Type 'quit' to exit.",
            "🤖".blue()
        );
        println!(
            "{} Ask me to install packages, suggest tools, or optimize your config.",
            "💡".green()
        );
        println!();

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("{} ", "❯".cyan());
            stdout.flush()?;

            let mut input = String::new();
            match stdin.lock().read_line(&mut input) {
                Ok(_) => {}
                Err(e) => {
                    println!("{} Error reading input: {}", "⚠".yellow(), e);
                    continue;
                }
            }

            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                println!("{} Goodbye!", "👋".blue());
                break;
            }

            if input.eq_ignore_ascii_case("help") {
                println!("\n{} Available commands:", "📖".blue());
                println!("  install <package> - Install a package");
                println!("  remove <package>  - Remove a package");
                println!("  suggest [topic]   - Get suggestions");
                println!("  optimize          - Analyze configuration");
                println!("  list              - List installed packages");
                println!("  quit/exit         - Exit chat mode");
                println!();
                continue;
            }

            // Process the input through AI interpreter
            let action = self.interpreter.interpret(input);

            match action {
                Ok(ai_action) => {
                    self.handle_chat_action(ai_action).await?;
                }
                Err(e) => {
                    println!("{} Could not understand: {}", "⚠".yellow(), e);
                }
            }
        }

        Ok(())
    }

    /// Handle an AI action in chat mode
    async fn handle_chat_action(&mut self, action: AiAction) -> Result<()> {
        match action.action {
            ActionType::Install => {
                println!(
                    "{} Installing: {}",
                    "📦".green(),
                    action.packages.join(", ")
                );
                for package in &action.packages {
                    if let Err(e) = self.install(package).await {
                        println!("{} Failed to install {}: {}", "⚠".yellow(), package, e);
                    }
                }
            }
            ActionType::Remove => {
                println!("{} Removing: {}", "🗑️".green(), action.packages.join(", "));
                for package in &action.packages {
                    if let Err(e) = self.remove(package).await {
                        println!("{} Failed to remove {}: {}", "⚠".yellow(), package, e);
                    }
                }
            }
            ActionType::Search => {
                println!(
                    "{} Searching for: {}",
                    "🔍".green(),
                    action.packages.join(", ")
                );
                for package in &action.packages {
                    if let Err(e) = self.search(package) {
                        println!("{} Search failed: {}", "⚠".yellow(), e);
                    }
                }
            }
            ActionType::Rollback => {
                println!(
                    "{} Rollback requested: {:?}",
                    "↩️".yellow(),
                    action.rollback_target
                );
                println!("   Use 'nixos-rebuild switch --rollback' for system rollback.");
            }
            ActionType::Optimize => {
                println!("{} Optimization requested.", "⚡".green());
                if let Err(e) = self.ai_optimize(true) {
                    println!("{} Optimization analysis failed: {}", "⚠".yellow(), e);
                }
            }
            ActionType::Suggest => {
                println!("{} Getting suggestions...", "💡".green());
                if let Err(e) = self.ai_suggest(None) {
                    println!("{} Suggestions failed: {}", "⚠".yellow(), e);
                }
            }
            ActionType::Unknown => {
                println!("{} Could not determine what to do.", "🤔".yellow());
                println!("   Try: 'install firefox' or 'suggest rust'");
            }
        }

        Ok(())
    }

    /// Semantic search for packages
    fn semantic_search(&self, query: &str) -> Result<()> {
        println!("{} Performing semantic search for: {}", "🔍".blue(), query);

        let engine = SemanticSearchEngine::new(self.resolver.clone());
        let results = engine.search(query);

        if results.is_empty() {
            println!("{}", "No packages found.".yellow());
            return Ok(());
        }

        println!(
            "Found {} package(s):\n",
            results.len().to_string().green().bold()
        );

        for (i, result) in results.iter().take(10).enumerate() {
            let match_icon = match result.match_type {
                slop::ai_search::MatchType::Exact => "🎯",
                slop::ai_search::MatchType::Semantic => "🧠",
                slop::ai_search::MatchType::Category => "📂",
                slop::ai_search::MatchType::Related => "🔗",
                slop::ai_search::MatchType::Fuzzy => "📝",
            };

            println!(
                "{}. {} {} {}",
                (i + 1).to_string().cyan().bold(),
                match_icon,
                result.attr_name.green(),
                result.reasoning.dimmed()
            );

            if let Some(desc) = &result.package.description {
                println!("   {}", desc.dimmed());
            }
        }

        if results.len() > 10 {
            println!("... and {} more", (results.len() - 10).to_string().dimmed());
        }

        Ok(())
    }

    /// Run the interactive setup wizard
    fn ai_setup(&self) -> Result<()> {
        let mut wizard = SetupWizard::new();
        wizard.set_verbose(self.verbose);

        let preferences = wizard.run()?;

        println!("\n{} Setup wizard completed!", "✓".green());
        println!(
            "{} Review your preferences above and run the suggested commands.",
            "💡".yellow()
        );

        Ok(())
    }

    /// Detect hardware and show recommendations
    fn ai_detect_hardware(&self) -> Result<()> {
        println!("{} Detecting hardware...", "🔍".blue());

        let mut detector = HardwareDetector::new();

        match detector.detect() {
            Ok(_) => {
                detector.print_hardware_info();
                detector.print_recommendations();
            }
            Err(e) => {
                println!("{} Hardware detection failed: {}", "⚠".yellow(), e);
                println!("{} Some recommendations may not be available.", "ℹ".blue());
            }
        }

        Ok(())
    }

    /// Check for package conflicts
    fn ai_check_conflicts(&self, packages: &[String]) -> Result<()> {
        println!("{} Checking for conflicts...", "🔍".blue());

        let detector = ConflictDetector::from_config("/etc/nixos/configuration.nix")
            .unwrap_or_else(|_| ConflictDetector::new());

        let report = detector.generate_report(packages);

        if report.total == 0 {
            println!("\n{} No conflicts detected!", "✓".green());
            return Ok(());
        }

        println!("\n{}", report.summary().bold());
        println!();

        for conflict in &report.conflicts {
            let severity_icon = match conflict.severity {
                slop::ai_conflicts::ConflictSeverity::Critical => "🔴",
                slop::ai_conflicts::ConflictSeverity::Error => "🔴",
                slop::ai_conflicts::ConflictSeverity::Warning => "🟡",
                slop::ai_conflicts::ConflictSeverity::Info => "ℹ️",
            };

            println!(
                "{} {} {}",
                severity_icon,
                conflict.title.bold(),
                "(#".dimmed(),
                conflict.id.dimmed(),
                ")".dimmed()
            );
            println!("   {}", conflict.description);
            println!("   {} {}", "💡".yellow(), conflict.suggestion);
            println!();
        }

        if report.is_safe() {
            println!("{} Installation can proceed safely.", "✓".green());
        } else {
            println!(
                "{} Please resolve critical issues before proceeding.",
                "⚠".yellow()
            );
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
        println!("{} Checking for package updates...", "🔄".blue());

        if self.dry_run {
            println!(
                "{} Would update all packages in environment.systemPackages",
                "→".yellow()
            );
            println!(
                "{} Would run: sudo nixos-rebuild switch --upgrade",
                "→".yellow()
            );
            return Ok(());
        }

        println!(
            "{} Note: Full package updates require running nixos-rebuild with --upgrade flag.",
            "ℹ".blue()
        );
        println!("{} Run: sudo nixos-rebuild switch --upgrade", "💡".yellow());

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
                    println!("{} Input '{}' already exists", "ℹ".blue(), name.yellow());
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
                    println!("{} Input '{}' not found", "ℹ".blue(), name.yellow());
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

    /// Run AI health check
    fn ai_health(&self) -> Result<()> {
        println!("{} Running system health check...", "🏥".blue());

        let checker = HealthChecker::new("/etc/nixos/configuration.nix");

        match checker.run_check() {
            Ok(report) => {
                checker.print_report(&report);
            }
            Err(e) => {
                println!("{} Health check failed: {}", "⚠".yellow(), e);
            }
        }

        Ok(())
    }

    /// Show conversation history
    fn ai_history(&self, limit: usize) -> Result<()> {
        println!("{} Loading conversation history...", "📜".blue());

        match AiMemory::new() {
            Ok(memory) => {
                let history = memory.get_history(Some(limit));

                if history.is_empty() {
                    println!("{} No conversation history found.", "ℹ".blue());
                    return Ok(());
                }

                println!("\n{} Recent conversations:\n", "📜".green());

                for msg in history.iter().rev() {
                    let role_icon = match msg.role {
                        slop::ai_memory::MessageRole::User => "👤",
                        slop::ai_memory::MessageRole::Assistant => "🤖",
                        slop::ai_memory::MessageRole::System => "⚙️",
                    };

                    println!("{} {}", role_icon, msg.content);

                    if !msg.packages.is_empty() {
                        println!("   Packages: {}", msg.packages.join(", ").cyan());
                    }
                    println!();
                }
            }
            Err(e) => {
                println!("{} Failed to load history: {}", "⚠".yellow(), e);
            }
        }

        Ok(())
    }

    /// Clear conversation history
    fn ai_clear_history(&self) -> Result<()> {
        use dialoguer::Confirm;

        println!("{} Clear conversation history?", "⚠️".yellow());

        if !Confirm::new()
            .with_prompt("Are you sure?")
            .default(false)
            .interact()?
        {
            println!("{} Cancelled.", "ℹ".blue());
            return Ok(());
        }

        match AiMemory::new() {
            Ok(mut memory) => {
                memory.clear_history()?;
                println!("{} Conversation history cleared.", "✓".green());
            }
            Err(e) => {
                println!("{} Failed to clear history: {}", "⚠".yellow(), e);
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
        let shell = shell
            .parse::<clap_complete::Shell>()
            .unwrap_or(clap_complete::Shell::Bash);

        clap_complete::generate(shell, &mut cmd, "slop", &mut io::stdout());

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
