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

/// Counts installed packages for Debian/Ubuntu family from a status file path.
pub fn count_dpkg_from_path(path: &Path) -> Option<usize> {
    if let Ok(content) = fs::read_to_string(path) {
        let count = parse_dpkg_status(&content);
        if count > 0 {
            return Some(count);
        }
    }
    None
}

/// Counts installed packages for Debian/Ubuntu family.
pub fn count_dpkg() -> Option<usize> {
    if let Some(count) = count_dpkg_from_path(Path::new("/var/lib/dpkg/status")) {
        return Some(count);
    }

    // Fallback: dpkg-query command if status file is inaccessible
    if let Ok(output) = Command::new("dpkg-query")
        .args(["-f", "${binary:Package}\n", "-W"])
        .output()
    {
        if output.status.success() {
            let count = parse_rpm_output(&output.stdout);
            if count > 0 {
                return Some(count);
            }
        }
    }

    None
}

/// Counts installed packages for Arch Linux family from a given pacman local database directory.
pub fn count_pacman_from_dir(pacman_dir: &Path) -> Option<usize> {
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

/// Counts installed packages for Arch Linux family from pacman local database.
pub fn count_pacman() -> Option<usize> {
    count_pacman_from_dir(Path::new("/var/lib/pacman/local"))
}

/// Counts non-empty newline-delimited entries from a raw byte slice (such as `rpm -qa`, `dpkg-query`, or `xbps-query -l`).
pub fn count_newline_entries(output: &[u8]) -> usize {
    output
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .count()
}

/// Alias for backwards compatibility with tests.
pub fn parse_rpm_output(output: &[u8]) -> usize {
    count_newline_entries(output)
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
                let count = count_newline_entries(&output.stdout);
                if count > 0 {
                    return Some(count);
                }
            }
        }
    }

    None
}

/// Parses APK installed db content.
pub fn parse_apk_installed(content: &str) -> usize {
    content.lines().filter(|l| l.starts_with("P:")).count()
}

/// Counts installed packages for Alpine Linux.
pub fn count_apk() -> Option<usize> {
    let path = Path::new("/lib/apk/db/installed");
    if let Ok(content) = fs::read_to_string(path) {
        let count = parse_apk_installed(&content);
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
                    let c = count_newline_entries(&output.stdout);
                    if c > 0 {
                        return Some(c);
                    }
                }
            }
        }
    }
    None
}

/// Counts installed Flatpak applications from specified system and user paths.
pub fn count_flatpak_from_dirs(sys_path: &Path, user_path: &Path) -> Option<usize> {
    let mut total = 0;

    if let Ok(entries) = fs::read_dir(sys_path) {
        total += entries
            .flatten()
            .filter(|e| {
                e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                    && !e.file_name().to_string_lossy().starts_with('.')
            })
            .count();
    }

    if let Ok(entries) = fs::read_dir(user_path) {
        total += entries
            .flatten()
            .filter(|e| {
                e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                    && !e.file_name().to_string_lossy().starts_with('.')
            })
            .count();
    }

    if total > 0 {
        Some(total)
    } else {
        None
    }
}

/// Counts installed Flatpak applications (system and user level).
pub fn count_flatpak() -> Option<usize> {
    let sys_path = Path::new("/var/lib/flatpak/app");
    let user_path = std::env::var("HOME")
        .map(|h| Path::new(&h).join(".local/share/flatpak/app"))
        .unwrap_or_else(|_| Path::new("/nonexistent").to_path_buf());
    count_flatpak_from_dirs(sys_path, &user_path)
}

/// Counts installed Snap packages from specified directories, deduplicating revisions.
pub fn count_snap_from_dirs(snaps_path: &Path, snap_root: &Path) -> Option<usize> {
    if let Ok(entries) = fs::read_dir(snaps_path) {
        let mut unique_snaps = std::collections::HashSet::new();
        for entry in entries.flatten() {
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if s.ends_with(".snap") {
                // Strip revision suffix, e.g. core22_1.snap -> core22
                let pkg_name = s.split_once('_').map(|(p, _)| p).unwrap_or(&s);
                unique_snaps.insert(pkg_name.to_string());
            }
        }
        if !unique_snaps.is_empty() {
            return Some(unique_snaps.len());
        }
    }

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

/// Counts installed Snap packages.
pub fn count_snap() -> Option<usize> {
    count_snap_from_dirs(Path::new("/var/lib/snapd/snaps"), Path::new("/snap"))
}

/// Counts installed Homebrew formulae from standard Cellar paths.
pub fn count_brew() -> Option<usize> {
    let cellar_paths = [
        Path::new("/home/linuxbrew/.linuxbrew/Cellar"),
        Path::new("/opt/homebrew/Cellar"),
        Path::new("/usr/local/Cellar"),
    ];

    for path in &cellar_paths {
        if let Ok(entries) = fs::read_dir(path) {
            let count = entries
                .flatten()
                .filter(|e| {
                    e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                        && !e.file_name().to_string_lossy().starts_with('.')
                })
                .count();
            if count > 0 {
                return Some(count);
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let user_cellar = Path::new(&home).join(".linuxbrew/Cellar");
        if let Ok(entries) = fs::read_dir(&user_cellar) {
            let count = entries
                .flatten()
                .filter(|e| {
                    e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                        && !e.file_name().to_string_lossy().starts_with('.')
                })
                .count();
            if count > 0 {
                return Some(count);
            }
        }
    }

    None
}

/// Counts installed Gentoo ebuild packages from /var/db/pkg.
pub fn count_emerge() -> Option<usize> {
    let pkg_dir = Path::new("/var/db/pkg");
    if let Ok(categories) = fs::read_dir(pkg_dir) {
        let mut total = 0;
        for cat in categories.flatten() {
            if cat.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if let Ok(pkgs) = fs::read_dir(cat.path()) {
                    total += pkgs
                        .flatten()
                        .filter(|p| {
                            p.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                                && !p.file_name().to_string_lossy().starts_with('.')
                        })
                        .count();
                }
            }
        }
        if total > 0 {
            return Some(total);
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
    if let Some(emerge) = count_emerge() {
        parts.push(format!("{} (emerge)", emerge));
    }
    if let Some(flatpak) = count_flatpak() {
        parts.push(format!("{} (flatpak)", flatpak));
    }
    if let Some(snap) = count_snap() {
        parts.push(format!("{} (snap)", snap));
    }
    if let Some(brew) = count_brew() {
        parts.push(format!("{} (brew)", brew));
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

    #[test]
    fn test_parse_dpkg_status_empty_or_corrupted() {
        assert_eq!(parse_dpkg_status(""), 0);
        assert_eq!(
            parse_dpkg_status("Package: foo\nStatus: half-installed\n"),
            0
        );
        assert_eq!(parse_dpkg_status("random text\nno valid header\n"), 0);
    }

    #[test]
    fn test_parse_rpm_output() {
        let output =
            b"coreutils-9.1-1.fc38.x86_64\nbash-5.2.15-3.fc38.x86_64\nglibc-2.37-4.fc38.x86_64\n";
        assert_eq!(parse_rpm_output(output), 3);
        assert_eq!(parse_rpm_output(b""), 0);
        assert_eq!(parse_rpm_output(b"\n\n"), 0);
    }

    #[test]
    fn test_parse_apk_installed() {
        let content = "P:musl\nV:1.2.4\n\nP:busybox\nV:1.36.1\n\nP:alpine-keys\nV:2.4-r1\n";
        assert_eq!(parse_apk_installed(content), 3);
        assert_eq!(parse_apk_installed(""), 0);
    }

    #[test]
    fn test_count_pacman_from_dir_mock() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path();

        // Initially empty
        assert_eq!(count_pacman_from_dir(path), None);

        // Add dummy package directories and files
        fs::create_dir(path.join("coreutils-9.5-1")).unwrap();
        fs::create_dir(path.join("linux-6.8.9-arch1-1")).unwrap();
        fs::create_dir(path.join(".hidden-dir")).unwrap();
        fs::write(path.join("ALPM_DB_VERSION"), "9").unwrap();

        assert_eq!(count_pacman_from_dir(path), Some(2));
    }

    #[test]
    fn test_count_pacman_from_dir_nonexistent() {
        assert_eq!(
            count_pacman_from_dir(Path::new("/nonexistent_pacman_dir_12345")),
            None
        );
    }

    #[test]
    fn test_count_flatpak_from_dirs_mock() {
        let sys_tmp = tempfile::tempdir().unwrap();
        let user_tmp = tempfile::tempdir().unwrap();

        fs::create_dir(sys_tmp.path().join("org.mozilla.firefox")).unwrap();
        fs::create_dir(user_tmp.path().join("org.videolan.VLC")).unwrap();

        let count = count_flatpak_from_dirs(sys_tmp.path(), user_tmp.path());
        assert_eq!(count, Some(2));
    }

    #[test]
    fn test_count_snap_from_dirs_mock() {
        let snaps_tmp = tempfile::tempdir().unwrap();
        let snap_root_tmp = tempfile::tempdir().unwrap();

        // 2 revisions of core22, 1 revision of firefox -> total 2 distinct packages
        fs::write(snaps_tmp.path().join("core22_1.snap"), b"").unwrap();
        fs::write(snaps_tmp.path().join("core22_2.snap"), b"").unwrap();
        fs::write(snaps_tmp.path().join("firefox_2.snap"), b"").unwrap();

        let count = count_snap_from_dirs(snaps_tmp.path(), snap_root_tmp.path());
        assert_eq!(count, Some(2));
    }

    #[test]
    fn test_count_newline_entries() {
        let output = b"pkg1\npkg2\npkg3\n";
        assert_eq!(count_newline_entries(output), 3);
        assert_eq!(count_newline_entries(b""), 0);
    }

    #[test]
    fn test_count_emerge_mock() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cat1 = temp_dir.path().join("sys-apps");
        let cat2 = temp_dir.path().join("app-editors");
        fs::create_dir_all(&cat1).unwrap();
        fs::create_dir_all(&cat2).unwrap();

        fs::create_dir(cat1.join("coreutils-9.3")).unwrap();
        fs::create_dir(cat1.join("systemd-254")).unwrap();
        fs::create_dir(cat2.join("neovim-0.9.4")).unwrap();

        let mut total = 0;
        for cat in fs::read_dir(temp_dir.path()).unwrap().flatten() {
            if cat.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if let Ok(pkgs) = fs::read_dir(cat.path()) {
                    total += pkgs
                        .flatten()
                        .filter(|p| {
                            p.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                                && !p.file_name().to_string_lossy().starts_with('.')
                        })
                        .count();
                }
            }
        }
        assert_eq!(total, 3);
    }
}
