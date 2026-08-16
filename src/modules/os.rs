use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    pub display_name: String,
    pub distro_id: String,
    pub distro_like: Vec<String>,
}

/// Parses standard `/etc/os-release` or `/usr/lib/os-release` file contents.
pub fn parse_os_release(content: &str) -> OsInfo {
    let mut pretty_name = None;
    let mut name = None;
    let mut version = None;
    let mut id = None;
    let mut id_like = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let mut val = v.trim();

            if val.len() >= 2
                && ((val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\'')))
            {
                val = &val[1..val.len() - 1];
            }
            let val = val.trim();
            if val.is_empty() {
                continue;
            }

            match key {
                "PRETTY_NAME" => pretty_name = Some(val.to_string()),
                "NAME" => name = Some(val.to_string()),
                "VERSION" | "VERSION_ID" => {
                    if version.is_none() {
                        version = Some(val.to_string());
                    }
                }
                "ID" => id = Some(val.to_lowercase()),
                "ID_LIKE" => id_like = Some(val.to_lowercase()),
                _ => {}
            }
        }
    }

    let display_name = pretty_name
        .or_else(|| match (name, version) {
            (Some(n), Some(v)) => Some(format!("{} {}", n, v)),
            (Some(n), None) => Some(n),
            _ => None,
        })
        .unwrap_or_else(|| "Linux".to_string());

    let distro_id = id.unwrap_or_else(|| "linux".to_string());
    let distro_like = id_like
        .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
        .unwrap_or_default();

    OsInfo {
        display_name,
        distro_id,
        distro_like,
    }
}

/// Detects the operating system using standard and legacy paths.
pub fn detect_os() -> OsInfo {
    // 1. Primary standard os-release files
    for path in &["/etc/os-release", "/usr/lib/os-release"] {
        if let Ok(content) = fs::read_to_string(path) {
            let info = parse_os_release(&content);
            if !info.display_name.is_empty() && info.display_name != "Linux" {
                return info;
            }
        }
    }

    // 2. Legacy distribution release files
    if let Ok(deb) = fs::read_to_string("/etc/debian_version") {
        let trimmed = deb.trim();
        if !trimmed.is_empty() {
            return OsInfo {
                display_name: format!("Debian {}", trimmed),
                distro_id: "debian".to_string(),
                distro_like: Vec::new(),
            };
        }
    }

    if let Ok(rh) = fs::read_to_string("/etc/redhat-release") {
        let trimmed = rh.trim();
        if !trimmed.is_empty() {
            return OsInfo {
                display_name: trimmed.to_string(),
                distro_id: "rhel".to_string(),
                distro_like: vec!["fedora".to_string()],
            };
        }
    }

    if Path::new("/etc/arch-release").exists() {
        return OsInfo {
            display_name: "Arch Linux".to_string(),
            distro_id: "arch".to_string(),
            distro_like: Vec::new(),
        };
    }

    if let Ok(gentoo) = fs::read_to_string("/etc/gentoo-release") {
        let trimmed = gentoo.trim();
        if !trimmed.is_empty() {
            return OsInfo {
                display_name: trimmed.to_string(),
                distro_id: "gentoo".to_string(),
                distro_like: Vec::new(),
            };
        }
    }

    if let Ok(alpine) = fs::read_to_string("/etc/alpine-release") {
        let trimmed = alpine.trim();
        if !trimmed.is_empty() {
            return OsInfo {
                display_name: format!("Alpine Linux {}", trimmed),
                distro_id: "alpine".to_string(),
                distro_like: Vec::new(),
            };
        }
    }

    // 3. Fallback to generic kernel sysname
    OsInfo {
        display_name: "Linux".to_string(),
        distro_id: "linux".to_string(),
        distro_like: Vec::new(),
    }
}

pub struct OsCollector;

impl Collector for OsCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Os
    }

    fn collect(&self, ctx: &FetchContext) -> Option<ModuleOutput> {
        Some(ModuleOutput {
            id: ModuleId::Os,
            label: "OS".to_string(),
            value: ctx.os_info.display_name.clone(),
            custom_rendered: None,
        })
    }
}

/// Detects hardware product/host model from sysfs or devicetree.
pub fn detect_host() -> Option<String> {
    // Check DMI product name and version
    let product_name = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .ok()
        .map(|s| s.trim().to_string());
    let product_version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
        .ok()
        .map(|s| s.trim().to_string());

    if let Some(ref name) = product_name {
        let name_lower = name.to_lowercase();
        let is_invalid = name_lower.is_empty()
            || name_lower == "none"
            || name_lower == "default string"
            || name_lower == "system product name"
            || name_lower == "to be filled by o.e.m."
            || name_lower.starts_with("system manufacturer");

        if !is_invalid {
            let mut full = name.clone();
            if let Some(ref ver) = product_version {
                let ver_lower = ver.to_lowercase();
                if !ver_lower.is_empty()
                    && ver_lower != "none"
                    && ver_lower != "default string"
                    && ver_lower != "to be filled by o.e.m."
                    && ver_lower != "1.0"
                    && ver_lower != name_lower
                {
                    full = format!("{} {}", name, ver);
                }
            }
            return Some(full);
        }
    }

    // Device tree model for ARM boards (Raspberry Pi, etc.)
    if let Ok(model) = fs::read("/sys/firmware/devicetree/base/model") {
        let clean = String::from_utf8_lossy(&model)
            .trim_matches('\0')
            .trim()
            .to_string();
        if !clean.is_empty() {
            return Some(clean);
        }
    }

    // Board name fallback
    if let Ok(board) = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name") {
        let clean = board.trim();
        if !clean.is_empty() && clean != "None" && clean != "Default string" {
            return Some(clean.to_string());
        }
    }

    None
}

pub struct HostCollector;

impl Collector for HostCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Host
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        detect_host().map(|host| ModuleOutput {
            id: ModuleId::Host,
            label: "Host".to_string(),
            value: host,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_os_release_ubuntu() {
        let fixture = r#"
NAME="Ubuntu"
VERSION="24.04 LTS (Noble Numbat)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 24.04 LTS"
VERSION_ID="24.04"
"#;
        let info = parse_os_release(fixture);
        assert_eq!(info.display_name, "Ubuntu 24.04 LTS");
        assert_eq!(info.distro_id, "ubuntu");
        assert_eq!(info.distro_like, vec!["debian"]);
    }

    #[test]
    fn test_parse_os_release_arch() {
        let fixture = r#"
NAME="Arch Linux"
PRETTY_NAME="Arch Linux"
ID=arch
BUILD_ID=rolling
"#;
        let info = parse_os_release(fixture);
        assert_eq!(info.display_name, "Arch Linux");
        assert_eq!(info.distro_id, "arch");
        assert!(info.distro_like.is_empty());
    }

    #[test]
    fn test_parse_os_release_fallback_name_version() {
        let fixture = r#"
NAME="CustomOS"
VERSION="1.0"
ID=custom
"#;
        let info = parse_os_release(fixture);
        assert_eq!(info.display_name, "CustomOS 1.0");
        assert_eq!(info.distro_id, "custom");
    }

    #[test]
    fn test_parse_os_release_empty_or_corrupted() {
        let empty_info = parse_os_release("");
        assert_eq!(empty_info.display_name, "Linux");
        assert_eq!(empty_info.distro_id, "linux");
        assert!(empty_info.distro_like.is_empty());

        let corrupted = "garbage text without equals\nrandom words\n";
        let corrupt_info = parse_os_release(corrupted);
        assert_eq!(corrupt_info.display_name, "Linux");
        assert_eq!(corrupt_info.distro_id, "linux");
    }

    #[test]
    fn test_parse_os_release_unquoted() {
        let unquoted = "NAME=CustomArch\nID=arch\nID_LIKE=arch\nPRETTY_NAME=Custom Arch Linux\n";
        let info = parse_os_release(unquoted);
        assert_eq!(info.display_name, "Custom Arch Linux");
        assert_eq!(info.distro_id, "arch");
        assert_eq!(info.distro_like, vec!["arch"]);
    }
}
