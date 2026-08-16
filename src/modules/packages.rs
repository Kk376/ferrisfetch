use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Parses Debian `/var/lib/dpkg/status` content and counts installed packages.
pub fn parse_dpkg_status(content: &str) -> usize {
    let mut count = 0;
    for line in content.lines() {
        if line.starts_with("Status:") && line.ends_with(" installed") {
            count += 1;
        }
    }
    count
}

/// Counts installed packages for Debian/Ubuntu family.
pub fn count_dpkg() -> Option<usize> {
    let path = Path::new("/var/lib/dpkg/status");
    if let Ok(content) = fs::read_to_string(path) {
        let count = parse_dpkg_status(&content);
        if count > 0 {
            return Some(count);
        }
    }
    None
}

/// Counts installed packages for Arch Linux family from pacman local database.
pub fn count_pacman() -> Option<usize> {
    let pacman_dir = Path::new("/var/lib/pacman/local");
    if let Ok(entries) = fs::read_dir(pacman_dir) {
        let count = entries
            .flatten()
            .filter(|e| {
                if let Ok(ft) = e.file_type() {
                    if ft.is_dir() {
                        let name = e.file_name();
                        let s = name.to_string_lossy();
                        return !s.starts_with('.') && s != "ALPM_DB_VERSION";
                    }
                }
                false
            })
            .count();
        if count > 0 {
            return Some(count);
        }
    }
    None
}

/// Counts installed packages for Red Hat / Fedora family via rpm.
pub fn count_rpm() -> Option<usize> {
    // Check if rpm databases exist first before invoking rpm
    let rpm_paths = [
        "/var/lib/rpm/Packages",
        "/var/lib/rpm/rpmdb.sqlite",
        "/usr/lib/sysimage/rpm/Packages",
        "/usr/lib/sysimage/rpm/rpmdb.sqlite",
    ];

    let has_rpm_db = rpm_paths.iter().any(|p| Path::new(p).exists());
    if has_rpm_db {
        if let Ok(output) = Command::new("rpm").arg("-qa").output() {
            if output.status.success() {
                let count = output
                    .stdout
                    .split(|&b| b == b'\n')
                    .filter(|l| !l.is_empty())
                    .count();
                if count > 0 {
                    return Some(count);
                }
            }
        }
    }

    None
}

/// Counts installed packages for Alpine Linux.
pub fn count_apk() -> Option<usize> {
    let path = Path::new("/lib/apk/db/installed");
    if let Ok(content) = fs::read_to_string(path) {
        let count = content.lines().filter(|l| l.starts_with("P:")).count();
        if count > 0 {
            return Some(count);
        }
    }
    None
}

/// Counts installed packages for Void Linux.
pub fn count_xbps() -> Option<usize> {
    let path = Path::new("/var/db/xbps");
    if let Ok(entries) = fs::read_dir(path) {
        let count = entries
            .flatten()
            .filter(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                s.starts_with("pkgdb")
            })
            .count();
        if count > 0 {
            if let Ok(output) = Command::new("xbps-query").arg("-l").output() {
                if output.status.success() {
                    let c = output
                        .stdout
                        .split(|&b| b == b'\n')
                        .filter(|l| !l.is_empty())
                        .count();
                    if c > 0 {
                        return Some(c);
                    }
                }
            }
        }
    }
    None
}

/// Counts installed Flatpak applications (system and user level).
pub fn count_flatpak() -> Option<usize> {
    let mut total = 0;

    let sys_path = Path::new("/var/lib/flatpak/app");
    if let Ok(entries) = fs::read_dir(sys_path) {
        total += entries
            .flatten()
            .filter(|e| {
                e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                    && !e.file_name().to_string_lossy().starts_with('.')
            })
            .count();
    }

    if let Ok(home) = std::env::var("HOME") {
        let user_path = Path::new(&home).join(".local/share/flatpak/app");
        if let Ok(entries) = fs::read_dir(user_path) {
            total += entries
                .flatten()
                .filter(|e| {
                    e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                        && !e.file_name().to_string_lossy().starts_with('.')
                })
                .count();
        }
    }

    if total > 0 {
        Some(total)
    } else {
        None
    }
}

/// Counts installed Snap packages.
pub fn count_snap() -> Option<usize> {
    let snap_dir = Path::new("/var/lib/snapd/snaps");
    if let Ok(entries) = fs::read_dir(snap_dir) {
        let count = entries
            .flatten()
            .filter(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                s.ends_with(".snap")
            })
            .count();
        if count > 0 {
            return Some(count);
        }
    }

    let snap_root = Path::new("/snap");
    if let Ok(entries) = fs::read_dir(snap_root) {
        let count = entries
            .flatten()
            .filter(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                s != "bin" && s != "README" && !s.starts_with('.')
            })
            .count();
        if count > 0 {
            return Some(count);
        }
    }

    None
}

/// Gathers and formats package counts across all active package managers.
pub fn get_packages_summary() -> Option<String> {
    let mut parts = Vec::new();

    if let Some(dpkg) = count_dpkg() {
        parts.push(format!("{} (dpkg)", dpkg));
    }
    if let Some(pacman) = count_pacman() {
        parts.push(format!("{} (pacman)", pacman));
    }
    if let Some(rpm) = count_rpm() {
        parts.push(format!("{} (rpm)", rpm));
    }
    if let Some(apk) = count_apk() {
        parts.push(format!("{} (apk)", apk));
    }
    if let Some(xbps) = count_xbps() {
        parts.push(format!("{} (xbps)", xbps));
    }
    if let Some(flatpak) = count_flatpak() {
        parts.push(format!("{} (flatpak)", flatpak));
    }
    if let Some(snap) = count_snap() {
        parts.push(format!("{} (snap)", snap));
    }

    if !parts.is_empty() {
        Some(parts.join(", "))
    } else {
        None
    }
}

pub struct PackagesCollector;

impl Collector for PackagesCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Packages
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let summary = get_packages_summary()?;
        Some(ModuleOutput {
            id: ModuleId::Packages,
            label: "Packages".to_string(),
            value: summary,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dpkg_status() {
        let fixture = r#"
Package: bash
Status: install ok installed
Section: shells

Package: curl
Status: deinstall ok config-files
Section: web

Package: libc6
Status: hold ok installed
Section: libs
"#;
        assert_eq!(parse_dpkg_status(fixture), 2);
    }
}
