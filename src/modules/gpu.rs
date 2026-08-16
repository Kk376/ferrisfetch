use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Maps PCI vendor hex IDs to human-readable manufacturer names.
/// Maps PCI vendor hex IDs to human-readable manufacturer names.
pub fn vendor_id_to_name(vendor: &str) -> Option<&'static str> {
    let clean = vendor
        .trim()
        .strip_prefix("0x")
        .or_else(|| vendor.trim().strip_prefix("0X"))
        .unwrap_or(vendor.trim())
        .to_lowercase();

    match clean.as_str() {
        "10de" => Some("NVIDIA"),
        "1002" => Some("AMD"),
        "8086" => Some("Intel"),
        "1af4" => Some("VirtIO GPU"),
        "1414" => Some("Microsoft Direct3D"),
        "15ad" => Some("VMware SVGA"),
        "80ee" => Some("VirtualBox Graphics"),
        "1013" => Some("Cirrus Logic"),
        "1234" => Some("QEMU VGA"),
        "13d7" => Some("Broadcom VideoCore"),
        "1a03" => Some("ASPEED Graphics"),
        "102b" => Some("Matrox Graphics"),
        "1b36" => Some("Red Hat QXL"),
        "5143" => Some("Qualcomm Adreno"),
        _ => None,
    }
}

/// Cleans redundant vendor suffixes and bracketed tags from GPU names.
pub fn clean_gpu_name(name: &str) -> String {
    let cleaned = name
        .replace("Corporation", "")
        .replace("Technologies Inc", "")
        .replace("Inc.", "")
        .replace("Inc", "")
        .replace("[AMD/ATI]", "")
        .replace("(rev a1)", "")
        .replace("(rev 02)", "");

    let tokens: Vec<&str> = cleaned.split_whitespace().collect();
    tokens.join(" ")
}

/// Probes a given PCI sysfs directory for display controllers (PCI class 0x03xxxx).
pub fn detect_gpus_from_sysfs_dir(pci_dir: &Path) -> Vec<String> {
    let mut gpus = Vec::new();

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
                        if !device_str.is_empty() && !device_str.starts_with("0x") {
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

/// Probes `/sys/bus/pci/devices/` for display controllers (PCI class 0x03xxxx).
pub fn detect_gpus_sysfs() -> Vec<String> {
    detect_gpus_from_sysfs_dir(Path::new("/sys/bus/pci/devices"))
}

/// Parses the output of `lspci -mm` for display devices.
pub fn parse_lspci_mm_output(text: &str) -> Vec<String> {
    let mut gpus = Vec::new();
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
            let raw_name = format!("{} {}", vendor, device);
            let cleaned = clean_gpu_name(&raw_name);
            let final_name = if !cleaned.is_empty() {
                cleaned
            } else {
                raw_name
            };
            if !gpus.contains(&final_name) {
                gpus.push(final_name);
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
                let parsed = parse_lspci_mm_output(&text);
                for gpu in parsed {
                    if !gpus.contains(&gpu) {
                        gpus.push(gpu);
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
        // If sysfs found generic/raw hex device IDs or generic names, try lspci to get full device names
        let has_raw_device_ids = sysfs_gpus
            .iter()
            .any(|g| g.contains("0x") || g.contains("PCI Display"));
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
        assert_eq!(vendor_id_to_name("10DE"), Some("NVIDIA"));
        assert_eq!(vendor_id_to_name("0x1002"), Some("AMD"));
        assert_eq!(vendor_id_to_name("0x8086"), Some("Intel"));
        assert_eq!(vendor_id_to_name("0x1414"), Some("Microsoft Direct3D"));
        assert_eq!(vendor_id_to_name("0x1af4"), Some("VirtIO GPU"));
        assert_eq!(vendor_id_to_name("0x15ad"), Some("VMware SVGA"));
        assert_eq!(vendor_id_to_name("0x9999"), None);
    }

    #[test]
    fn test_parse_lspci_mm_output() {
        let text = r#"
00:02.0 "VGA compatible controller" "Intel Corporation" "CometLake-H GT2 [UHD Graphics]" -r02 "Dell" "Device 099f"
01:00.0 "3D controller" "NVIDIA Corporation" "TU117M [GeForce GTX 1650 Ti Mobile]" -ra1 "Dell" "Device 099f"
"#;
        let parsed = parse_lspci_mm_output(text);
        assert_eq!(parsed.len(), 2);
        assert!(parsed[0].contains("Intel CometLake-H GT2 [UHD Graphics]"));
        assert!(parsed[1].contains("NVIDIA TU117M [GeForce GTX 1650 Ti Mobile]"));
    }

    #[test]
    fn test_detect_gpus_from_sysfs_dir_mock() {
        let temp_dir = tempfile::tempdir().unwrap();
        let pci_dir = temp_dir.path();

        // GPU 1: Intel
        let gpu1 = pci_dir.join("0000:00:02.0");
        fs::create_dir_all(&gpu1).unwrap();
        fs::write(gpu1.join("class"), "0x030000\n").unwrap();
        fs::write(gpu1.join("vendor"), "0x8086\n").unwrap();
        fs::write(gpu1.join("device"), "0x9bc4\n").unwrap();

        // GPU 2: NVIDIA with label
        let gpu2 = pci_dir.join("0000:01:00.0");
        fs::create_dir_all(&gpu2).unwrap();
        fs::write(gpu2.join("class"), "0x030200\n").unwrap();
        fs::write(gpu2.join("vendor"), "0x10de\n").unwrap();
        fs::write(gpu2.join("device"), "0x1f95\n").unwrap();
        fs::write(gpu2.join("label"), "NVIDIA GeForce GTX 1650 Ti\n").unwrap();

        // Non-display device (Network)
        let net = pci_dir.join("0000:02:00.0");
        fs::create_dir_all(&net).unwrap();
        fs::write(net.join("class"), "0x020000\n").unwrap();
        fs::write(net.join("vendor"), "0x8086\n").unwrap();

        let gpus = detect_gpus_from_sysfs_dir(pci_dir);
        assert_eq!(gpus.len(), 2);
        assert!(gpus.contains(&"Intel".to_string()));
        assert!(gpus.contains(&"NVIDIA GeForce GTX 1650 Ti".to_string()));
    }

    #[test]
    fn test_virtio_and_hyperv_sysfs_mock() {
        let temp_dir = tempfile::tempdir().unwrap();
        let pci_dir = temp_dir.path();

        let virtio = pci_dir.join("0000:00:01.0");
        fs::create_dir_all(&virtio).unwrap();
        fs::write(virtio.join("class"), "0x030000\n").unwrap();
        fs::write(virtio.join("vendor"), "0x1af4\n").unwrap();

        let gpus = detect_gpus_from_sysfs_dir(pci_dir);
        assert_eq!(gpus, vec!["VirtIO GPU".to_string()]);
    }
}
