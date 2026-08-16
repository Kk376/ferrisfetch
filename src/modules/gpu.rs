use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Maps PCI vendor hex IDs to human-readable manufacturer names.
pub fn vendor_id_to_name(vendor: &str) -> Option<&'static str> {
    let clean = vendor.trim().to_lowercase();
    match clean.as_str() {
        "0x10de" | "10de" => Some("NVIDIA"),
        "0x1002" | "1002" => Some("AMD"),
        "0x8086" | "8086" => Some("Intel"),
        "0x1af4" | "1af4" => Some("VirtIO GPU"),
        "0x1414" | "1414" => Some("Microsoft Direct3D"),
        "0x15ad" | "15ad" => Some("VMware SVGA"),
        "0x80ee" | "80ee" => Some("VirtualBox Graphics"),
        "0x1013" | "1013" => Some("Cirrus Logic"),
        "0x1234" | "1234" => Some("QEMU VGA"),
        "0x13d7" | "13d7" => Some("Broadcom VideoCore"),
        "0x1a03" | "1a03" => Some("ASPEED Graphics"),
        "0x102b" | "102b" => Some("Matrox Graphics"),
        _ => None,
    }
}

/// Probes `/sys/bus/pci/devices/` for display controllers (PCI class 0x03xxxx).
pub fn detect_gpus_sysfs() -> Vec<String> {
    let mut gpus = Vec::new();
    let pci_dir = Path::new("/sys/bus/pci/devices");

    if let Ok(entries) = fs::read_dir(pci_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let class_path = path.join("class");
            if let Ok(class_str) = fs::read_to_string(class_path) {
                let class_trimmed = class_str.trim().to_lowercase();
                // Class 0x03xxxx corresponds to VGA (0x0300), 3D (0x0302), and Display (0x0380)
                if class_trimmed.starts_with("0x03") || class_trimmed.starts_with("3") {
                    let vendor_str = fs::read_to_string(path.join("vendor"))
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    let device_str = fs::read_to_string(path.join("device"))
                        .unwrap_or_default()
                        .trim()
                        .to_string();

                    let label = fs::read_to_string(path.join("label"))
                        .ok()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());

                    let vendor_name = vendor_id_to_name(&vendor_str);

                    let gpu_name = if let Some(lbl) = label {
                        lbl
                    } else if let Some(v_name) = vendor_name {
                        if !device_str.is_empty() {
                            format!("{} ({})", v_name, device_str)
                        } else {
                            v_name.to_string()
                        }
                    } else if !vendor_str.is_empty() {
                        format!("PCI Display Controller ({}:{})", vendor_str, device_str)
                    } else {
                        "Display Controller".to_string()
                    };

                    if !gpus.contains(&gpu_name) {
                        gpus.push(gpu_name);
                    }
                }
            }
        }
    }

    gpus
}

/// Fallback probe using `lspci -mm` when sysfs does not yield specific models.
pub fn detect_gpus_lspci() -> Vec<String> {
    let mut gpus = Vec::new();
    for class_filter in &["::0300", "::0302", "::0380"] {
        if let Ok(output) = Command::new("lspci")
            .args(["-mm", "-d", class_filter])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    // Format: Slot "Class" "Vendor" "Device" "SVendor" "SDevice"
                    let parts: Vec<&str> = trimmed.split('"').collect();
                    if parts.len() >= 7 {
                        let vendor = parts[3].trim();
                        let device = parts[5].trim();
                        let name = format!("{} {}", vendor, device);
                        if !gpus.contains(&name) {
                            gpus.push(name);
                        }
                    }
                }
            }
        }
    }
    gpus
}

pub fn get_gpu_info() -> Option<String> {
    let sysfs_gpus = detect_gpus_sysfs();
    if !sysfs_gpus.is_empty() {
        // If sysfs found generic/raw hex device IDs, try lspci to get full device names
        let has_raw_device_ids = sysfs_gpus.iter().any(|g| g.contains("0x"));
        if has_raw_device_ids {
            let lspci_gpus = detect_gpus_lspci();
            if !lspci_gpus.is_empty() {
                return Some(lspci_gpus.join(", "));
            }
        }
        return Some(sysfs_gpus.join(", "));
    }

    let lspci_gpus = detect_gpus_lspci();
    if !lspci_gpus.is_empty() {
        return Some(lspci_gpus.join(", "));
    }

    None
}

pub struct GpuCollector;

impl Collector for GpuCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Gpu
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let value = get_gpu_info()?;
        Some(ModuleOutput {
            id: ModuleId::Gpu,
            label: "GPU".to_string(),
            value,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_id_mapping() {
        assert_eq!(vendor_id_to_name("0x10de"), Some("NVIDIA"));
        assert_eq!(vendor_id_to_name("0x1002"), Some("AMD"));
        assert_eq!(vendor_id_to_name("0x8086"), Some("Intel"));
        assert_eq!(vendor_id_to_name("0x1414"), Some("Microsoft Direct3D"));
        assert_eq!(vendor_id_to_name("0x9999"), None);
    }
}
