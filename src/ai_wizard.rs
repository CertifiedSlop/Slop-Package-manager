//! AI Setup Wizard
//!
//! Interactive guided setup for new NixOS users.

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

/// Setup step type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetupStep {
    SystemType,
    DesktopEnvironment,
    DevelopmentStack,
    EssentialTools,
    MediaApps,
    Review,
}

/// User preferences collected during setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub system_type: SystemType,
    pub desktop: Option<DesktopEnvironment>,
    pub dev_languages: Vec<String>,
    pub essential_tools: Vec<String>,
    pub media_apps: Vec<String>,
    pub use_flakes: bool,
    pub use_home_manager: bool,
}

/// System type selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemType {
    Desktop,
    Server,
    Development,
    MediaCenter,
    Minimal,
}

/// Desktop environment options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DesktopEnvironment {
    GNOME,
    KDE,
    XFCE,
    I3,
    Sway,
    Hyprland,
    None,
}

/// Setup wizard
pub struct SetupWizard {
    preferences: UserPreferences,
    verbose: bool,
}

impl SetupWizard {
    /// Create a new setup wizard
    pub fn new() -> Self {
        SetupWizard {
            preferences: UserPreferences {
                system_type: SystemType::Desktop,
                desktop: None,
                dev_languages: Vec::new(),
                essential_tools: Vec::new(),
                media_apps: Vec::new(),
                use_flakes: true,
                use_home_manager: true,
            },
            verbose: false,
        }
    }

    /// Set verbose mode
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Run the interactive setup wizard
    pub fn run(&mut self) -> Result<UserPreferences> {
        println!("{} Welcome to the Slop Setup Wizard!", "🤖".blue());
        println!(
            "{} Let me help you configure your NixOS system.\n",
            "💡".green()
        );

        self.step_system_type()?;

        if self.preferences.system_type == SystemType::Desktop
            || self.preferences.system_type == SystemType::Development
            || self.preferences.system_type == SystemType::MediaCenter
        {
            self.step_desktop_environment()?;
        }

        self.step_development_stack()?;
        self.step_essential_tools()?;

        if self.preferences.system_type == SystemType::Desktop
            || self.preferences.system_type == SystemType::MediaCenter
        {
            self.step_media_apps()?;
        }

        self.step_advanced_options()?;
        self.step_review()?;

        Ok(self.preferences.clone())
    }

    /// Step 1: Determine system type
    fn step_system_type(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 1: System Type", "📋".blue());
        println!(
            "{}\n",
            "What will you use this system for?".to_string().bold()
        );

        let options = [
            ("Desktop daily driver", SystemType::Desktop),
            ("Development machine", SystemType::Development),
            ("Server / Headless", SystemType::Server),
            ("Media center / HTPC", SystemType::MediaCenter),
            ("Minimal / Custom", SystemType::Minimal),
        ];

        for (i, (desc, _)) in options.iter().enumerate() {
            println!("  {}) {}", (i + 1).to_string().cyan(), desc);
        }

        let choice = self.get_number_input(1, options.len(), 1)?;
        self.preferences.system_type = options[choice - 1].1.clone();

        println!(
            "\n{} Selected: {}",
            "✓".green(),
            format!("{:?}", self.preferences.system_type).green()
        );

        Ok(())
    }

    /// Step 2: Desktop environment (if applicable)
    fn step_desktop_environment(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 2: Desktop Environment", "🖥️".blue());
        println!(
            "{}\n",
            "Which desktop environment do you prefer?"
                .to_string()
                .bold()
        );

        let options = [
            ("GNOME - Polished, user-friendly", DesktopEnvironment::GNOME),
            ("KDE Plasma - Highly customizable", DesktopEnvironment::KDE),
            ("XFCE - Lightweight, traditional", DesktopEnvironment::XFCE),
            (
                "i3 - Tiling window manager (keyboard-focused)",
                DesktopEnvironment::I3,
            ),
            ("Sway - Tiling Wayland compositor", DesktopEnvironment::Sway),
            (
                "Hyprland - Modern Wayland compositor",
                DesktopEnvironment::Hyprland,
            ),
            ("None - Window manager only", DesktopEnvironment::None),
        ];

        for (i, (desc, _)) in options.iter().enumerate() {
            println!("  {}) {}", (i + 1).to_string().cyan(), desc);
        }

        let choice = self.get_number_input(1, options.len(), 1)?;
        self.preferences.desktop = Some(options[choice - 1].1.clone());

        println!(
            "\n{} Selected: {}",
            "✓".green(),
            format!("{:?}", self.preferences.desktop.as_ref().unwrap()).green()
        );

        // Provide tips based on selection
        match self.preferences.desktop.as_ref().unwrap() {
            DesktopEnvironment::GNOME => {
                println!(
                    "\n{} GNOME works best with extensions. I'll suggest some later.",
                    "💡".yellow()
                );
            }
            DesktopEnvironment::KDE => {
                println!(
                    "\n{} KDE offers many customization options. Great for power users!",
                    "💡".yellow()
                );
            }
            DesktopEnvironment::I3 | DesktopEnvironment::Sway => {
                println!(
                    "\n{} Tiling WMs have a learning curve but boost productivity!",
                    "💡".yellow()
                );
            }
            _ => {}
        }

        Ok(())
    }

    /// Step 3: Development stack
    fn step_development_stack(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 3: Development Stack", "💻".blue());
        println!(
            "{}\n",
            "Which programming languages will you use? (comma-separated)"
                .to_string()
                .bold()
        );

        let available = vec![
            ("rust", "Rust - Systems programming"),
            ("python", "Python - General purpose"),
            ("javascript", "JavaScript/TypeScript - Web development"),
            ("go", "Go - Backend services"),
            ("cpp", "C/C++ - Systems programming"),
            ("java", "Java - Enterprise applications"),
            ("none", "Skip development tools"),
        ];

        println!("Available options:");
        for (lang, desc) in &available {
            println!("  • {} - {}", lang.to_string().cyan(), desc.dimmed());
        }

        let input = self.get_text_input()?;
        let selections: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        for selection in &selections {
            if selection == "none" {
                self.preferences.dev_languages = Vec::new();
                break;
            }
            if available.iter().any(|(lang, _)| lang == selection) {
                self.preferences.dev_languages.push(selection.clone());
            }
        }

        if !self.preferences.dev_languages.is_empty() {
            println!(
                "\n{} Selected: {}",
                "✓".green(),
                self.preferences.dev_languages.join(", ").green()
            );
        } else {
            println!("\n{} No development languages selected", "ℹ".blue());
        }

        Ok(())
    }

    /// Step 4: Essential tools
    fn step_essential_tools(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 4: Essential Tools", "🔧".blue());
        println!(
            "{}\n",
            "Which essential tools do you need? (comma-separated)"
                .to_string()
                .bold()
        );

        let available = vec![
            ("browser", "Web browser (Firefox)"),
            ("editor", "Text editor (Neovim)"),
            ("terminal", "Terminal emulator (Alacritty)"),
            ("shell", "Custom shell (Zsh)"),
            ("git", "Version control (Git)"),
            ("cli-tools", "CLI power tools (ripgrep, fd, fzf, bat)"),
            ("system-monitor", "System monitoring (htop, btop)"),
            ("none", "Skip essential tools"),
        ];

        println!("Available options:");
        for (tool, desc) in &available {
            println!("  • {} - {}", tool.to_string().cyan(), desc.dimmed());
        }

        let input = self.get_text_input()?;
        let selections: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        for selection in &selections {
            if selection == "none" {
                self.preferences.essential_tools = Vec::new();
                break;
            }
            if available.iter().any(|(tool, _)| tool == selection) {
                self.preferences.essential_tools.push(selection.clone());
            }
        }

        if !self.preferences.essential_tools.is_empty() {
            println!(
                "\n{} Selected: {}",
                "✓".green(),
                self.preferences.essential_tools.join(", ").green()
            );
        }

        Ok(())
    }

    /// Step 5: Media applications
    fn step_media_apps(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 5: Media Applications", "🎵".blue());
        println!(
            "{}\n",
            "Which media applications do you need? (comma-separated)"
                .to_string()
                .bold()
        );

        let available = vec![
            ("video-player", "Video player (VLC, MPV)"),
            ("music-player", "Music player (Spotify)"),
            ("image-viewer", "Image viewer (nomacs)"),
            ("image-editor", "Image editor (GIMP)"),
            ("video-editor", "Video editor (Kdenlive)"),
            ("streaming", "Streaming software (OBS)"),
            ("none", "Skip media apps"),
        ];

        println!("Available options:");
        for (app, desc) in &available {
            println!("  • {} - {}", app.to_string().cyan(), desc.dimmed());
        }

        let input = self.get_text_input()?;
        let selections: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        for selection in &selections {
            if selection == "none" {
                self.preferences.media_apps = Vec::new();
                break;
            }
            if available.iter().any(|(app, _)| app == selection) {
                self.preferences.media_apps.push(selection.clone());
            }
        }

        if !self.preferences.media_apps.is_empty() {
            println!(
                "\n{} Selected: {}",
                "✓".green(),
                self.preferences.media_apps.join(", ").green()
            );
        }

        Ok(())
    }

    /// Step 6: Advanced options
    fn step_advanced_options(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 6: Advanced Options", "⚙️".blue());
        println!();

        // Flakes option
        println!("{} Enable Nix Flakes?", "❓".yellow());
        println!("  1) Yes (recommended for new users)");
        println!("  2) No (use traditional channels)");

        let flakes_choice = self.get_number_input(1, 2, 1)?;
        self.preferences.use_flakes = flakes_choice == 1;

        println!(
            "\n{} Nix Flakes: {}",
            "✓".green(),
            if self.preferences.use_flakes {
                "Enabled"
            } else {
                "Disabled"
            }
            .green()
        );

        // Home-manager option
        println!("\n{} Enable Home-Manager?", "❓".yellow());
        println!("  1) Yes (manage user dotfiles)");
        println!("  2) No (system packages only)");

        let hm_choice = self.get_number_input(1, 2, 1)?;
        self.preferences.use_home_manager = hm_choice == 1;

        println!(
            "\n{} Home-Manager: {}",
            "✓".green(),
            if self.preferences.use_home_manager {
                "Enabled"
            } else {
                "Disabled"
            }
            .green()
        );

        Ok(())
    }

    /// Step 7: Review and confirm
    fn step_review(&mut self) -> Result<()> {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Step 7: Review Configuration", "📋".blue());
        println!(
            "\n{}\n",
            "Here's your configuration summary:".to_string().bold()
        );

        println!(
            "{} System Type: {}",
            "📋".blue(),
            format!("{:?}", self.preferences.system_type).cyan()
        );

        if let Some(ref de) = self.preferences.desktop {
            println!("{} Desktop: {}", "🖥️".blue(), format!("{:?}", de).cyan());
        }

        if !self.preferences.dev_languages.is_empty() {
            println!(
                "{} Development: {}",
                "💻".blue(),
                self.preferences.dev_languages.join(", ").cyan()
            );
        }

        if !self.preferences.essential_tools.is_empty() {
            println!(
                "{} Essential Tools: {}",
                "🔧".blue(),
                self.preferences.essential_tools.join(", ").cyan()
            );
        }

        if !self.preferences.media_apps.is_empty() {
            println!(
                "{} Media Apps: {}",
                "🎵".blue(),
                self.preferences.media_apps.join(", ").cyan()
            );
        }

        println!(
            "{} Nix Flakes: {}",
            "⚙️".blue(),
            if self.preferences.use_flakes {
                "Yes"
            } else {
                "No"
            }
            .to_string()
            .cyan()
        );
        println!(
            "{} Home-Manager: {}",
            "🏠".blue(),
            if self.preferences.use_home_manager {
                "Yes"
            } else {
                "No"
            }
            .to_string()
            .cyan()
        );

        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("\n{} Proceed with this configuration?", "❓".yellow());
        println!("  1) Yes, apply configuration");
        println!("  2) No, start over");
        println!("  3) No, exit");

        let choice = self.get_number_input(1, 3, 1)?;

        match choice {
            1 => {
                println!("\n{} Configuration confirmed!", "✓".green());
                self.show_next_steps();
            }
            2 => {
                println!("\n{} Starting over...", "🔄".yellow());
                self.run()?;
                return Ok(());
            }
            _ => {
                println!("\n{} Setup cancelled.", "ℹ".blue());
            }
        }

        Ok(())
    }

    /// Show next steps after setup
    fn show_next_steps(&mut self) {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Next Steps", "📝".blue());
        println!("{}", "═══════════════════════════════════════".dimmed());

        println!("\n{} Your configuration will include:\n", "📦".green());

        // Generate package list based on preferences
        let mut packages = Vec::new();

        // Essential tools
        if self
            .preferences
            .essential_tools
            .contains(&"browser".to_string())
        {
            packages.push("firefox".to_string());
        }
        if self
            .preferences
            .essential_tools
            .contains(&"editor".to_string())
        {
            packages.push("neovim".to_string());
        }
        if self
            .preferences
            .essential_tools
            .contains(&"terminal".to_string())
        {
            packages.push("alacritty".to_string());
        }
        if self
            .preferences
            .essential_tools
            .contains(&"shell".to_string())
        {
            packages.push("zsh".to_string());
        }
        if self
            .preferences
            .essential_tools
            .contains(&"git".to_string())
        {
            packages.push("git".to_string());
        }
        if self
            .preferences
            .essential_tools
            .contains(&"cli-tools".to_string())
        {
            packages.extend(vec![
                "ripgrep".to_string(),
                "fd".to_string(),
                "fzf".to_string(),
                "bat".to_string(),
            ]);
        }
        if self
            .preferences
            .essential_tools
            .contains(&"system-monitor".to_string())
        {
            packages.extend(vec!["htop".to_string(), "btop".to_string()]);
        }

        // Development languages
        if self.preferences.dev_languages.contains(&"rust".to_string()) {
            packages.extend(vec!["rustup".to_string(), "cargo".to_string()]);
        }
        if self
            .preferences
            .dev_languages
            .contains(&"python".to_string())
        {
            packages.extend(vec!["python3".to_string(), "pip".to_string()]);
        }
        if self
            .preferences
            .dev_languages
            .contains(&"javascript".to_string())
        {
            packages.extend(vec!["nodejs".to_string(), "npm".to_string()]);
        }
        if self.preferences.dev_languages.contains(&"go".to_string()) {
            packages.push("go".to_string());
        }

        // Media apps
        if self
            .preferences
            .media_apps
            .contains(&"video-player".to_string())
        {
            packages.extend(vec!["vlc".to_string(), "mpv".to_string()]);
        }
        if self
            .preferences
            .media_apps
            .contains(&"music-player".to_string())
        {
            packages.push("spotify".to_string());
        }

        println!("Packages to install:");
        for pkg in &packages {
            println!("  {} {}", "•".green(), pkg);
        }

        println!("\n{} To apply this configuration, run:\n", "💡".yellow());
        println!("  {}", "slop install <packages>".cyan());
        println!("\nOr let me install them for you:\n");
        println!(
            "  {}",
            format!("slop ai install {}", packages.join(" ")).cyan()
        );

        println!("\n{} Happy NixOSing!", "🎉".blue());
    }

    /// Get number input from user
    fn get_number_input(&self, min: usize, max: usize, default: usize) -> Result<usize> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("{} ", "❯".cyan());
            stdout.flush()?;

            let mut input = String::new();
            stdin.lock().read_line(&mut input)?;

            let input = input.trim();

            if input.is_empty() {
                return Ok(default);
            }

            if let Ok(num) = input.parse::<usize>() {
                if num >= min && num <= max {
                    return Ok(num);
                }
            }

            println!(
                "{} Please enter a number between {} and {}",
                "⚠".yellow(),
                min.to_string().cyan(),
                max.to_string().cyan()
            );
        }
    }

    /// Get text input from user
    fn get_text_input(&self) -> Result<String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        print!("{} ", "❯".cyan());
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate Nix configuration from preferences
pub fn generate_nix_config(preferences: &UserPreferences) -> String {
    let mut config = String::new();

    config.push_str("{ config, pkgs, ... }: {\n\n");

    // System type specific settings
    match preferences.system_type {
        SystemType::Desktop | SystemType::Development => {
            config.push_str("  # Desktop configuration\n");
        }
        SystemType::Server => {
            config.push_str("  # Server configuration\n");
            config.push_str("  services.openssh.enable = true;\n\n");
        }
        _ => {}
    }

    // Desktop environment
    if let Some(ref de) = preferences.desktop {
        match de {
            DesktopEnvironment::GNOME => {
                config.push_str("  # GNOME Desktop\n");
                config.push_str("  services.xserver.enable = true;\n");
                config.push_str("  services.xserver.displayManager.gdm.enable = true;\n");
                config.push_str("  services.xserver.desktopManager.gnome.enable = true;\n\n");
            }
            DesktopEnvironment::KDE => {
                config.push_str("  # KDE Plasma Desktop\n");
                config.push_str("  services.xserver.enable = true;\n");
                config.push_str("  services.xserver.displayManager.sddm.enable = true;\n");
                config.push_str("  services.xserver.desktopManager.plasma5.enable = true;\n\n");
            }
            DesktopEnvironment::I3 => {
                config.push_str("  # i3 Window Manager\n");
                config.push_str("  services.xserver.enable = true;\n");
                config.push_str("  services.xserver.desktopManager.xterm.enable = false;\n");
                config.push_str("  services.xserver.windowManager.i3.enable = true;\n\n");
            }
            DesktopEnvironment::Sway => {
                config.push_str("  # Sway Window Manager (Wayland)\n");
                config.push_str("  programs.sway.enable = true;\n\n");
            }
            _ => {}
        }
    }

    // Development packages
    let mut packages = Vec::new();

    if preferences.dev_languages.contains(&"rust".to_string()) {
        packages.extend(vec!["rustup", "cargo", "rustfmt", "clippy"]);
    }
    if preferences.dev_languages.contains(&"python".to_string()) {
        packages.extend(vec!["python3", "pip"]);
    }
    if preferences
        .dev_languages
        .contains(&"javascript".to_string())
    {
        packages.extend(vec!["nodejs", "npm"]);
    }
    if preferences.dev_languages.contains(&"go".to_string()) {
        packages.push("go");
    }

    // Essential tools
    if preferences.essential_tools.contains(&"browser".to_string()) {
        packages.push("firefox");
    }
    if preferences.essential_tools.contains(&"editor".to_string()) {
        packages.push("neovim");
    }
    if preferences.essential_tools.contains(&"git".to_string()) {
        packages.push("git");
    }
    if preferences
        .essential_tools
        .contains(&"cli-tools".to_string())
    {
        packages.extend(vec!["ripgrep", "fd", "fzf", "bat"]);
    }

    // Media apps
    if preferences.media_apps.contains(&"video-player".to_string()) {
        packages.extend(vec!["vlc", "mpv"]);
    }
    if preferences.media_apps.contains(&"music-player".to_string()) {
        packages.push("spotify");
    }

    // Add packages section
    if !packages.is_empty() {
        config.push_str("  environment.systemPackages = with pkgs; [\n");
        for pkg in &packages {
            config.push_str(&format!("    {}\n", pkg));
        }
        config.push_str("  ];\n\n");
    }

    // Flakes option
    if preferences.use_flakes {
        config.push_str("  # Enable flakes\n");
        config
            .push_str("  nix.settings.experimental-features = [ \"nix-command\" \"flakes\" ];\n\n");
    }

    // Home-manager option
    if preferences.use_home_manager {
        config.push_str("  # Home-Manager will be configured separately\n");
        config.push_str("  # Run: slop home-manager init\n\n");
    }

    config.push_str("  system.stateVersion = \"23.11\";\n");
    config.push_str("}\n");

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_creation() {
        let wizard = SetupWizard::new();
        assert_eq!(wizard.preferences.system_type, SystemType::Desktop);
        assert!(wizard.preferences.use_flakes);
        assert!(wizard.preferences.use_home_manager);
    }

    #[test]
    fn test_generate_nix_config() {
        let preferences = UserPreferences {
            system_type: SystemType::Desktop,
            desktop: Some(DesktopEnvironment::GNOME),
            dev_languages: vec!["rust".to_string()],
            essential_tools: vec!["browser".to_string(), "editor".to_string()],
            media_apps: Vec::new(),
            use_flakes: true,
            use_home_manager: true,
        };

        let config = generate_nix_config(&preferences);

        assert!(config.contains("GNOME"));
        assert!(config.contains("rustup"));
        assert!(config.contains("firefox"));
        assert!(config.contains("flakes"));
    }
}
