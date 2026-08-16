use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

const KNOWN_WMS: &[&str] = &[
    "i3", "bspwm", "awesome", "dwm", "openbox", "xmonad", "qtile", "mutter", "kwin", "xfwm4",
    "compiz", "marco", "sway", "hyprland", "wayfire", "river", "labwc",
];

/// Probes the system for active Desktop Environment (DE), Window Manager (WM), and session type.
pub fn detect_desktop() -> Option<String> {
    let mut de = None;
    let mut wm = None;

    // 1. Detect Desktop Environment
    if let Ok(cur_de) = std::env::var("XDG_CURRENT_DESKTOP") {
        let clean = cur_de.trim();
        if !clean.is_empty() {
            // Handle colon-separated lists like "ubuntu:GNOME" or "pop:GNOME"
            let primary = clean.split(':').next_back().unwrap_or(clean);
            de = Some(primary.to_string());
        }
    } else if let Ok(sess) = std::env::var("DESKTOP_SESSION") {
        let clean = sess.trim();
        if !clean.is_empty() && clean != "default" {
            de = Some(clean.to_string());
        }
    }

    // 2. Detect Window Manager via Wayland signatures
    if std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some() {
        wm = Some("Hyprland".to_string());
    } else if std::env::var_os("SWAYSOCK").is_some() {
        wm = Some("Sway".to_string());
    } else if std::env::var_os("WAYFIRE_CONFIG_FILE").is_some() {
        wm = Some("Wayfire".to_string());
    } else if std::env::var_os("RIVER_SOCKET").is_some() {
        wm = Some("River".to_string());
    } else if std::env::var_os("LABWC_PID").is_some() {
        wm = Some("labwc".to_string());
    }

    // 3. Fallback WM check from running processes if WM is not yet identified
    if wm.is_none() {
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.chars().all(|c| c.is_ascii_digit()) {
                    let comm_path = entry.path().join("comm");
                    if let Ok(comm) = fs::read_to_string(comm_path) {
                        let clean_comm = comm.trim().to_lowercase();
                        for &known in KNOWN_WMS {
                            if clean_comm == known {
                                wm = Some(capitalize_first(known));
                                break;
                            }
                        }
                        if wm.is_some() {
                            break;
                        }
                    }
                }
            }
        }
    }

    // 4. Session type (Wayland / X11 / TTY)
    let session_type = std::env::var("XDG_SESSION_TYPE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "tty");

    // Format output
    match (de, wm, session_type) {
        (Some(d), Some(w), Some(sess)) => {
            if d.to_lowercase() == w.to_lowercase()
                || (d.to_lowercase() == "gnome" && w.to_lowercase() == "mutter")
                || (d.to_lowercase() == "kde" && w.to_lowercase() == "kwin")
            {
                Some(format!("{} ({})", d, capitalize_first(&sess)))
            } else {
                Some(format!("{} (WM: {}, {})", d, w, capitalize_first(&sess)))
            }
        }
        (Some(d), Some(w), None) => {
            if d.to_lowercase() == w.to_lowercase() {
                Some(d)
            } else {
                Some(format!("{} (WM: {})", d, w))
            }
        }
        (Some(d), None, Some(sess)) => Some(format!("{} ({})", d, capitalize_first(&sess))),
        (Some(d), None, None) => Some(d),
        (None, Some(w), Some(sess)) => Some(format!("{} ({})", w, capitalize_first(&sess))),
        (None, Some(w), None) => Some(w),
        (None, None, _) => None,
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => {
            if s.eq_ignore_ascii_case("x11") {
                "X11".to_string()
            } else if s.eq_ignore_ascii_case("wayland") {
                "Wayland".to_string()
            } else {
                first.to_uppercase().collect::<String>() + chars.as_str()
            }
        }
        None => String::new(),
    }
}

pub struct DesktopCollector;

impl Collector for DesktopCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Desktop
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let de = detect_desktop()?;
        Some(ModuleOutput {
            id: ModuleId::Desktop,
            label: "Desktop".to_string(),
            value: de,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("wayland"), "Wayland");
        assert_eq!(capitalize_first("x11"), "X11");
        assert_eq!(capitalize_first("gnome"), "Gnome");
    }
}
