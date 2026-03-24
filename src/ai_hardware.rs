//! AI Hardware Detection
//!
//! Detects hardware and suggests optimal configurations.

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Hardware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: CpuInfo,
    pub gpu: Vec<GpuInfo>,
    pub ram: RamInfo,
    pub storage: Vec<StorageInfo>,
    pub network: Vec<NetworkInfo>,
    pub audio: Vec<AudioInfo>,
}

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub vendor: String,
    pub model: String,
    pub cores: usize,
    pub threads: usize,
}

/// GPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub vendor: GpuVendor,
    pub model: String,
    pub vram: Option<String>,
}

/// GPU vendor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GpuVendor {
    NVIDIA,
    AMD,
    Intel,
    Unknown,
}

/// RAM information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamInfo {
    pub total_gb: f64,
}

/// Storage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub device: String,
    pub size_gb: f64,
    pub mount_point: Option<String>,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub driver: Option<String>,
    pub wireless: bool,
}

/// Audio information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    pub name: String,
    pub driver: String,
}

/// Hardware recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRecommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub packages: Vec<String>,
    pub config_snippet: Option<String>,
    pub priority: Priority,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
    Optional,
}

/// Hardware detector
pub struct HardwareDetector {
    hardware: Option<HardwareInfo>,
}

impl HardwareDetector {
    /// Create a new hardware detector
    pub fn new() -> Self {
        HardwareDetector { hardware: None }
    }

    /// Detect hardware
    pub fn detect(&mut self) -> Result<HardwareInfo> {
        let hardware = HardwareInfo {
            cpu: self.detect_cpu()?,
            gpu: self.detect_gpu()?,
            ram: self.detect_ram()?,
            storage: self.detect_storage()?,
            network: self.detect_network()?,
            audio: self.detect_audio()?,
        };

        self.hardware = Some(hardware.clone());
        Ok(hardware)
    }

    /// Detect CPU information
    fn detect_cpu(&self) -> Result<CpuInfo> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo")
            .context("Failed to read /proc/cpuinfo")?;

        let mut vendor = String::from("Unknown");
        let mut model = String::from("Unknown");
        let mut cores = 0;
        let mut threads = 0;

        for line in cpuinfo.lines() {
            if line.starts_with("vendor_id") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    vendor = parts[1].trim().to_string();
                }
            }
            if line.starts_with("model name") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    model = parts[1].trim().to_string();
                }
            }
            if line.starts_with("cpu cores") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    cores = parts[1].trim().parse().unwrap_or(1);
                }
            }
            if line.starts_with("siblings") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    threads = parts[1].trim().parse().unwrap_or(1);
                }
            }
        }

        // Count physical cores if not found
        if cores == 0 {
            cores = cpuinfo.matches("cpu cores").count();
        }

        // Count threads from processor entries
        if threads == 0 {
            threads = cpuinfo.matches("processor").count();
        }

        Ok(CpuInfo {
            vendor,
            model,
            cores: cores.max(1),
            threads: threads.max(1),
        })
    }

    /// Detect GPU information
    fn detect_gpu(&self) -> Result<Vec<GpuInfo>> {
        let mut gpus = Vec::new();

        // Try lspci for GPU detection
        if let Ok(output) = std::process::Command::new("lspci")
            .args(["-vnn"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                let line_lower = line.to_lowercase();

                if line_lower.contains("vga") || line_lower.contains("3d") || line_lower.contains("display") {
                    let vendor = if line.contains("NVIDIA") {
                        GpuVendor::NVIDIA
                    } else if line.contains("AMD") || line.contains("ATI") {
                        GpuVendor::AMD
                    } else if line.contains("Intel") {
                        GpuVendor::Intel
                    } else {
                        GpuVendor::Unknown
                    };

                    // Extract model name
                    let model = line
                        .split(':')
                        .last()
                        .unwrap_or("Unknown GPU")
                        .trim()
                        .to_string();

                    gpus.push(GpuInfo {
                        vendor,
                        model,
                        vram: None,
                    });
                }
            }
        }

        // Fallback: check for NVIDIA in /sys
        if gpus.is_empty() {
            if Path::new("/sys/bus/pci/drivers/nvidia").exists() {
                gpus.push(GpuInfo {
                    vendor: GpuVendor::NVIDIA,
                    model: "NVIDIA GPU".to_string(),
                    vram: None,
                });
            }
        }

        Ok(gpus)
    }

    /// Detect RAM information
    fn detect_ram(&self) -> Result<RamInfo> {
        let meminfo = fs::read_to_string("/proc/meminfo")
            .context("Failed to read /proc/meminfo")?;

        let mut total_kb = 0;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    total_kb = parts[1].parse().unwrap_or(0);
                }
                break;
            }
        }

        let total_gb = total_kb as f64 / 1024.0 / 1024.0;

        Ok(RamInfo { total_gb })
    }

    /// Detect storage information
    fn detect_storage(&self) -> Result<Vec<StorageInfo>> {
        let mut storage = Vec::new();

        // Try to read from /proc/partitions
        if let Ok(partitions) = fs::read_to_string("/proc/partitions") {
            for line in partitions.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let device = parts[3].to_string();
                    let blocks: u64 = parts[2].parse().unwrap_or(0);
                    let size_gb = blocks as f64 / 1024.0 / 1024.0;

                    // Only include significant devices (> 1GB)
                    if size_gb > 1.0 {
                        storage.push(StorageInfo {
                            device,
                            size_gb,
                            mount_point: None,
                        });
                    }
                }
            }
        }

        Ok(storage)
    }

    /// Detect network information
    fn detect_network(&self) -> Result<Vec<NetworkInfo>> {
        let mut networks = Vec::new();

        // Check /sys/class/net for network interfaces
        if let Ok(entries) = fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let interface = entry.file_name().to_string_lossy().to_string();

                // Skip loopback
                if interface == "lo" {
                    continue;
                }

                // Check if wireless
                let wireless = Path::new(&format!("/sys/class/net/{}/wireless", interface)).exists();

                // Try to get driver
                let driver = fs::read_link(&format!("/sys/class/net/{}/device/driver", interface))
                    .ok()
                    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()));

                networks.push(NetworkInfo {
                    interface,
                    driver,
                    wireless,
                });
            }
        }

        Ok(networks)
    }

    /// Detect audio information
    fn detect_audio(&self) -> Result<Vec<AudioInfo>> {
        let mut audio = Vec::new();

        // Try aplay -l for audio devices
        if let Ok(output) = std::process::Command::new("aplay")
            .arg("-l")
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                if line.starts_with("card") {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        audio.push(AudioInfo {
                            name: parts[1].trim().to_string(),
                            driver: "ALSA".to_string(),
                        });
                    }
                }
            }
        }

        // Fallback: check /proc/asound
        if audio.is_empty() && Path::new("/proc/asound/cards").exists() {
            if let Ok(cards) = fs::read_to_string("/proc/asound/cards") {
                for line in cards.lines() {
                    if line.contains(":") {
                        audio.push(AudioInfo {
                            name: line.trim().to_string(),
                            driver: "ALSA".to_string(),
                        });
                    }
                }
            }
        }

        // Default if nothing found
        if audio.is_empty() {
            audio.push(AudioInfo {
                name: "Default Audio Device".to_string(),
                driver: "Unknown".to_string(),
            });
        }

        Ok(audio)
    }

    /// Get hardware recommendations based on detected hardware
    pub fn get_recommendations(&self) -> Vec<HardwareRecommendation> {
        let mut recommendations = Vec::new();

        let Some(hardware) = &self.hardware else {
            return recommendations;
        };

        // GPU recommendations
        for gpu in &hardware.gpu {
            match gpu.vendor {
                GpuVendor::NVIDIA => {
                    recommendations.push(HardwareRecommendation {
                        category: "GPU Driver".to_string(),
                        title: "NVIDIA Proprietary Driver".to_string(),
                        description: "NVIDIA GPU detected. Install proprietary drivers for best performance.".to_string(),
                        packages: vec![
                            "nvidia-x11".to_string(),
                            "nvidia-settings".to_string(),
                        ],
                        config_snippet: Some(
                            "  hardware.nvidia.open = false;\n  hardware.nvidia.modesetting.enable = true;".to_string()
                        ),
                        priority: Priority::High,
                    });
                }
                GpuVendor::AMD => {
                    recommendations.push(HardwareRecommendation {
                        category: "GPU Driver".to_string(),
                        title: "AMD GPU Support".to_string(),
                        description: "AMD GPU detected. Open-source drivers are included in the kernel.".to_string(),
                        packages: vec![
                            "vulkan-loader".to_string(),
                            "vulkan-tools".to_string(),
                        ],
                        config_snippet: Some(
                            "  hardware.opengl.enable = true;\n  hardware.opengl.driSupport = true;".to_string()
                        ),
                        priority: Priority::Medium,
                    });
                }
                GpuVendor::Intel => {
                    recommendations.push(HardwareRecommendation {
                        category: "GPU Driver".to_string(),
                        title: "Intel GPU Support".to_string(),
                        description: "Intel GPU detected. Install media drivers for hardware acceleration.".to_string(),
                        packages: vec![
                            "intel-media-driver".to_string(),
                            "intel-compute-runtime".to_string(),
                            "vulkan-loader".to_string(),
                        ],
                        config_snippet: Some(
                            "  hardware.opengl.enable = true;\n  hardware.opengl.driSupport = true;".to_string()
                        ),
                        priority: Priority::Medium,
                    });
                }
                GpuVendor::Unknown => {}
            }
        }

        // RAM-based recommendations
        if hardware.ram.total_gb < 4.0 {
            recommendations.push(HardwareRecommendation {
                category: "Memory Optimization".to_string(),
                title: "Enable ZRAM".to_string(),
                description: format!(
                    "System has only {:.1}GB RAM. Enable ZRAM for better memory management.",
                    hardware.ram.total_gb
                ),
                packages: vec![],
                config_snippet: Some("  zramSwap.enable = true;".to_string()),
                priority: Priority::High,
            });
        } else if hardware.ram.total_gb >= 16.0 {
            recommendations.push(HardwareRecommendation {
                category: "Memory Optimization".to_string(),
                title: "Consider Zswap".to_string(),
                description: format!(
                    "System has {:.1}GB RAM. Zswap can improve performance.",
                    hardware.ram.total_gb
                ),
                packages: vec![],
                config_snippet: Some("  zswap.enabled = true;".to_string()),
                priority: Priority::Low,
            });
        }

        // Network recommendations
        for network in &hardware.network {
            if network.wireless {
                recommendations.push(HardwareRecommendation {
                    category: "Network".to_string(),
                    title: "WiFi Firmware".to_string(),
                    description: format!("Wireless adapter detected: {}. Ensure firmware is installed.", network.interface),
                    packages: vec![
                        "linux-firmware".to_string(),
                        "iw".to_string(),
                        "wpa_supplicant".to_string(),
                    ],
                    config_snippet: Some(
                        "  networking.networkmanager.enable = true;\n  networking.wireless.enable = true;".to_string()
                    ),
                    priority: Priority::High,
                });
            }
        }

        // Audio recommendations
        if !hardware.audio.is_empty() {
            recommendations.push(HardwareRecommendation {
                category: "Audio".to_string(),
                title: "PipeWire Audio".to_string(),
                description: "Enable PipeWire for modern audio support.".to_string(),
                packages: vec![
                    "pipewire".to_string(),
                    "wireplumber".to_string(),
                    "pipewire.pulse".to_string(),
                ],
                config_snippet: Some(
                    "  security.rtkit.enable = true;\n  services.pipewire.enable = true;\n  services.pipewire.pulse.enable = true;".to_string()
                ),
                priority: Priority::Medium,
            });
        }

        // CPU microcode recommendations
        if hardware.cpu.vendor.contains("GenuineIntel") || hardware.cpu.vendor.contains("Intel") {
            recommendations.push(HardwareRecommendation {
                category: "CPU".to_string(),
                title: "Intel CPU Microcode".to_string(),
                description: "Install Intel CPU microcode updates for security and stability.".to_string(),
                packages: vec!["intel-microcode".to_string()],
                config_snippet: Some("  hardware.cpu.intel.updateMicrocode = true;".to_string()),
                priority: Priority::High,
            });
        } else if hardware.cpu.vendor.contains("AuthenticAMD") || hardware.cpu.vendor.contains("AMD") {
            recommendations.push(HardwareRecommendation {
                category: "CPU".to_string(),
                title: "AMD CPU Microcode".to_string(),
                description: "Install AMD CPU microcode updates for security and stability.".to_string(),
                packages: vec!["linux-firmware".to_string()],
                config_snippet: Some("  hardware.cpu.amd.updateMicrocode = true;".to_string()),
                priority: Priority::High,
            });
        }

        // Sort by priority
        recommendations.sort_by(|a, b| {
            let priority_order = |p: &Priority| match p {
                Priority::High => 0,
                Priority::Medium => 1,
                Priority::Low => 2,
                Priority::Optional => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });

        recommendations
    }

    /// Print hardware information
    pub fn print_hardware_info(&self) {
        let Some(hardware) = &self.hardware else {
            println!("{} Hardware not detected yet. Run detect() first.", "⚠".yellow());
            return;
        };

        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Hardware Information", "🔍".blue());
        println!("{}\n", "═══════════════════════════════════════".dimmed());

        // CPU
        println!("{} CPU: {} {}", "📦".blue(), hardware.cpu.vendor, hardware.cpu.model);
        println!("   Cores: {}, Threads: {}", hardware.cpu.cores, hardware.cpu.threads);

        // GPU
        println!("\n{} GPU(s):", "🎮".blue());
        for gpu in &hardware.gpu {
            let vendor_str = match gpu.vendor {
                GpuVendor::NVIDIA => "NVIDIA",
                GpuVendor::AMD => "AMD",
                GpuVendor::Intel => "Intel",
                GpuVendor::Unknown => "Unknown",
            };
            println!("   • {} - {}", vendor_str, gpu.model);
        }

        // RAM
        println!("\n{} RAM: {:.1} GB", "💾".blue(), hardware.ram.total_gb);

        // Storage
        println!("\n{} Storage:", "💽".blue());
        for storage in &hardware.storage {
            println!("   • {} - {:.1} GB", storage.device, storage.size_gb);
        }

        // Network
        println!("\n{} Network:", "🌐".blue());
        for network in &hardware.network {
            let wifi_str = if network.wireless { "(WiFi)" } else { "(Ethernet)" };
            println!("   • {} {}", network.interface, wifi_str);
        }

        // Audio
        println!("\n{} Audio:", "🔊".blue());
        for audio in &hardware.audio {
            println!("   • {}", audio.name);
        }

        println!();
    }

    /// Print recommendations
    pub fn print_recommendations(&self) {
        let recommendations = self.get_recommendations();

        if recommendations.is_empty() {
            println!("{} No specific hardware recommendations.", "ℹ".blue());
            return;
        }

        println!("\n{}", "═══════════════════════════════════════".dimmed());
        println!("{} Hardware Recommendations", "💡".blue());
        println!("{}\n", "═══════════════════════════════════════".dimmed());

        for (i, rec) in recommendations.iter().enumerate() {
            let priority_icon = match rec.priority {
                Priority::High => "🔴",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
                Priority::Optional => "⚪",
            };

            println!("{}. {} {}", (i + 1).to_string().cyan().bold(), priority_icon, rec.title.bold());
            println!("   {}", rec.description);

            if !rec.packages.is_empty() {
                println!("   Packages: {}", rec.packages.join(", ").green());
            }

            if let Some(snippet) = &rec.config_snippet {
                println!("   Config:\n{}", snippet.dimmed());
            }

            println!();
        }
    }
}

impl Default for HardwareDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = HardwareDetector::new();
        assert!(detector.hardware.is_none());
    }

    #[test]
    fn test_recommendations_empty() {
        let detector = HardwareDetector::new();
        let recommendations = detector.get_recommendations();
        assert!(recommendations.is_empty());
    }
}
