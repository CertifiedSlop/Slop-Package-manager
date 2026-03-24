//! AI Health Check
//!
//! System health analysis and maintenance recommendations.

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub timestamp: u64,
    pub overall_score: u8,
    pub categories: Vec<CategoryHealth>,
    pub recommendations: Vec<Recommendation>,
    pub summary: String,
}

/// Category health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryHealth {
    pub name: String,
    pub score: u8,
    pub status: HealthStatus,
    pub details: Vec<String>,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Good,
    Warning,
    Critical,
}

/// Maintenance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub command: Option<String>,
    pub priority: Priority,
    pub estimated_impact: String,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Health checker
pub struct HealthChecker {
    config_path: String,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new(config_path: &str) -> Self {
        HealthChecker {
            config_path: config_path.to_string(),
        }
    }

    /// Run a full health check
    pub fn run_check(&self) -> Result<HealthReport> {
        let mut categories = Vec::new();
        let mut recommendations = Vec::new();
        let mut total_score = 0;

        // Check packages
        let packages_health = self.check_packages()?;
        total_score += packages_health.score;
        categories.push(packages_health);

        // Check disk usage
        let disk_health = self.check_disk_usage()?;
        total_score += disk_health.score;
        categories.push(disk_health);

        // Check security
        let security_health = self.check_security()?;
        total_score += security_health.score;
        categories.push(security_health);

        // Check system services
        let services_health = self.check_services()?;
        total_score += services_health.score;
        categories.push(services_health);

        // Check configuration
        let config_health = self.check_configuration()?;
        total_score += config_health.score;
        categories.push(config_health);

        // Calculate overall score
        let overall_score = (total_score / categories.len() as u8).min(100);

        // Generate summary
        let summary = self.generate_summary(&categories, overall_score);

        Ok(HealthReport {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            overall_score,
            categories,
            recommendations,
            summary,
        })
    }

    /// Check package status
    fn check_packages(&self) -> Result<CategoryHealth> {
        let mut details = Vec::new();
        let mut score = 100;

        // Check for outdated packages (simulated)
        if let Ok(output) = Command::new("nix-collect-garbage")
            .arg("--dry-run")
            .output()
        {
            if output.status.success() {
                details.push("Garbage collection check passed".to_string());
            }
        } else {
            details.push("Unable to check garbage collection status".to_string());
            score -= 10;
        }

        // Check /nix/store size
        if let Ok(entries) = fs::read_dir("/nix/store") {
            let count = entries.count();
            details.push(format!("{} packages in /nix/store", count));

            if count > 1000 {
                details.push("Consider running nix-collect-garbage".to_string());
                score -= 15;
            }
        }

        let status = if score >= 80 {
            HealthStatus::Good
        } else if score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        Ok(CategoryHealth {
            name: "Packages".to_string(),
            score,
            status,
            details,
        })
    }

    /// Check disk usage
    fn check_disk_usage(&self) -> Result<CategoryHealth> {
        let mut details = Vec::new();
        let mut score = 100;

        // Check root filesystem
        if let Ok(output) = Command::new("df").args(["-h", "/"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let usage = parts[4].trim_end_matches('%');
                    if let Ok(usage) = usage.parse::<u8>() {
                        details.push(format!("Root filesystem: {}% used", usage));

                        if usage > 90 {
                            details
                                .push("Critical: Less than 10% disk space remaining!".to_string());
                            score -= 40;
                        } else if usage > 80 {
                            details.push("Warning: Less than 20% disk space remaining".to_string());
                            score -= 20;
                        } else if usage > 70 {
                            details.push("Consider cleaning up old generations".to_string());
                            score -= 10;
                        }
                    }
                }
            }
        }

        // Check /nix/store size
        if let Ok(output) = Command::new("du").args(["-sh", "/nix/store"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let size = output_str.split_whitespace().next().unwrap_or("unknown");
            details.push(format!("/nix/store size: {}", size));
        }

        let status = if score >= 80 {
            HealthStatus::Good
        } else if score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        Ok(CategoryHealth {
            name: "Disk Usage".to_string(),
            score,
            status,
            details,
        })
    }

    /// Check security status
    fn check_security(&self) -> Result<CategoryHealth> {
        let mut details = Vec::new();
        let mut score = 100;

        // Check for security updates (simulated)
        details.push("Security check initiated".to_string());

        // Check if system is up to date
        if Path::new("/run/current-system").exists() {
            details.push("System generation is active".to_string());
        } else {
            details.push("Warning: No active system generation found".to_string());
            score -= 30;
        }

        // Check for sudo configuration
        if Path::new("/etc/sudoers").exists() {
            details.push("Sudo is configured".to_string());
        }

        // Check firewall status
        if let Ok(output) = Command::new("systemctl")
            .args(["is-active", "firewalld"])
            .output()
        {
            let status = String::from_utf8_lossy(&output.stdout).trim();
            if status == "active" {
                details.push("Firewall is active".to_string());
            } else {
                details.push("Firewall may not be configured".to_string());
                score -= 10;
            }
        }

        let status = if score >= 80 {
            HealthStatus::Good
        } else if score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        Ok(CategoryHealth {
            name: "Security".to_string(),
            score,
            status,
            details,
        })
    }

    /// Check system services
    fn check_services(&self) -> Result<CategoryHealth> {
        let mut details = Vec::new();
        let mut score = 100;

        // Check critical services
        let critical_services = ["NetworkManager", "sshd", "systemd-journald"];

        for service in &critical_services {
            if let Ok(output) = Command::new("systemctl")
                .args(["is-active", service])
                .output()
            {
                let status = String::from_utf8_lossy(&output.stdout).trim();
                if status == "active" || status == "inactive" {
                    details.push(format!("{}: {}", service, status));
                } else {
                    details.push(format!("{}: {} (potential issue)", service, status));
                    score -= 10;
                }
            }
        }

        let status = if score >= 80 {
            HealthStatus::Good
        } else if score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        Ok(CategoryHealth {
            name: "Services".to_string(),
            score,
            status,
            details,
        })
    }

    /// Check configuration health
    fn check_configuration(&self) -> Result<CategoryHealth> {
        let mut details = Vec::new();
        let mut score = 100;

        // Check if config file exists
        if Path::new(&self.config_path).exists() {
            details.push("Configuration file exists".to_string());

            // Check config file size
            if let Ok(metadata) = fs::metadata(&self.config_path) {
                let size = metadata.len();
                if size > 100_000 {
                    details.push("Configuration file is large (>100KB)".to_string());
                    details.push("Consider modularizing your configuration".to_string());
                    score -= 10;
                }
            }

            // Check for common issues
            if let Ok(content) = fs::read_to_string(&self.config_path) {
                if content.contains("TODO") || content.contains("FIXME") {
                    details.push("Configuration contains TODO/FIXME comments".to_string());
                    score -= 5;
                }

                if !content.contains("system.stateVersion") {
                    details.push("Warning: system.stateVersion not set".to_string());
                    score -= 20;
                }
            }
        } else {
            details.push("Configuration file not found".to_string());
            score -= 50;
        }

        let status = if score >= 80 {
            HealthStatus::Good
        } else if score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        Ok(CategoryHealth {
            name: "Configuration".to_string(),
            score,
            status,
            details,
        })
    }

    /// Generate summary
    fn generate_summary(&self, categories: &[CategoryHealth], overall_score: u8) -> String {
        let mut summary = String::new();

        summary.push_str(&format!("Overall Health Score: {}/100\n\n", overall_score));

        for category in categories {
            let icon = match category.status {
                HealthStatus::Good => "✅",
                HealthStatus::Warning => "⚠️",
                HealthStatus::Critical => "❌",
            };
            summary.push_str(&format!(
                "{} {}: {}/100\n",
                icon, category.name, category.score
            ));
        }

        summary
    }

    /// Get maintenance recommendations
    pub fn get_recommendations(&self, report: &HealthReport) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        for category in &report.categories {
            match category.name.as_str() {
                "Disk Usage" => {
                    if category.status == HealthStatus::Warning
                        || category.status == HealthStatus::Critical
                    {
                        recommendations.push(Recommendation {
                            category: "Disk Usage".to_string(),
                            title: "Clean up old generations".to_string(),
                            description: "Remove old system generations to free up disk space."
                                .to_string(),
                            command: Some("nix-collect-garbage --delete-older-than 7d".to_string()),
                            priority: Priority::High,
                            estimated_impact: "Can free 1-10GB of disk space".to_string(),
                        });
                    }
                }
                "Packages" => {
                    if category
                        .details
                        .iter()
                        .any(|d| d.contains("nix-collect-garbage"))
                    {
                        recommendations.push(Recommendation {
                            category: "Packages".to_string(),
                            title: "Run garbage collection".to_string(),
                            description: "Clean up unused packages from the Nix store.".to_string(),
                            command: Some("nix-collect-garbage -d".to_string()),
                            priority: Priority::Medium,
                            estimated_impact: "Variable disk space savings".to_string(),
                        });
                    }
                }
                "Security" => {
                    if category.status == HealthStatus::Warning {
                        recommendations.push(Recommendation {
                            category: "Security".to_string(),
                            title: "Review security configuration".to_string(),
                            description: "Some security settings may need attention.".to_string(),
                            command: None,
                            priority: Priority::Medium,
                            estimated_impact: "Improved system security".to_string(),
                        });
                    }
                }
                "Configuration" => {
                    if category.score < 80 {
                        recommendations.push(Recommendation {
                            category: "Configuration".to_string(),
                            title: "Review configuration health".to_string(),
                            description: "Configuration file may need attention or optimization."
                                .to_string(),
                            command: Some("slop ai-optimize".to_string()),
                            priority: Priority::Low,
                            estimated_impact: "Better maintainability".to_string(),
                        });
                    }
                }
                _ => {}
            }
        }

        recommendations
    }

    /// Print health report
    pub fn print_report(&self, report: &HealthReport) {
        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} System Health Report", "🏥".blue());
        println!("{}\n", "═══════════════════════════════════════".dimmed());

        let score_color = if report.overall_score >= 80 {
            &report.overall_score.to_string().green()
        } else if report.overall_score >= 50 {
            &report.overall_score.to_string().yellow()
        } else {
            &report.overall_score.to_string().red()
        };

        println!("Overall Score: {}\n", score_color.bold());

        for category in &report.categories {
            let icon = match category.status {
                HealthStatus::Good => "✅",
                HealthStatus::Warning => "⚠️",
                HealthStatus::Critical => "❌",
            };

            println!("{} {} ({}/100)", icon, category.name.bold(), category.score);

            for detail in &category.details {
                println!("   • {}", detail.dimmed());
            }
            println!();
        }

        // Print recommendations
        let recommendations = self.get_recommendations(report);

        if !recommendations.is_empty() {
            println!("{} Recommendations:\n", "💡".yellow());

            for rec in &recommendations {
                let priority_icon = match rec.priority {
                    Priority::Critical => "🔴",
                    Priority::High => "🟠",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢",
                };

                println!(
                    "{} {} {}",
                    priority_icon,
                    rec.title.bold(),
                    rec.category.dimmed()
                );
                println!("   {}", rec.description);

                if let Some(cmd) = &rec.command {
                    println!("   Run: {}", cmd.cyan());
                }

                println!("   Impact: {}", rec.estimated_impact.green());
                println!();
            }
        } else {
            println!("{} No immediate actions required.", "✓".green());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checker_creation() {
        let checker = HealthChecker::new("/etc/nixos/configuration.nix");
        assert_eq!(checker.config_path, "/etc/nixos/configuration.nix");
    }

    #[test]
    fn test_recommendations_generation() {
        let checker = HealthChecker::new("/etc/nixos/configuration.nix");

        let report = HealthReport {
            timestamp: 0,
            overall_score: 70,
            categories: vec![CategoryHealth {
                name: "Disk Usage".to_string(),
                score: 60,
                status: HealthStatus::Warning,
                details: vec!["Root filesystem: 85% used".to_string()],
            }],
            recommendations: Vec::new(),
            summary: String::new(),
        };

        let recommendations = checker.get_recommendations(&report);
        assert!(!recommendations.is_empty());
    }
}
