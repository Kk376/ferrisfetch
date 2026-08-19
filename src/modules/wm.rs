use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

const KNOWN_WMS: &[&str] = &[
    "kwin_wayland",
    "kwin_x11",
    "kwin",
    "mutter",
    "gnome-shell",
    "xfwm4",
    "muffin",
    "marco",
    "sway",
    "hyprland",
    "i3",
    "bspwm",
    "dwm",
    "awesome",
    "xmonad",
    "qtile",
    "openbox",
    "fluxbox",
    "enlightenment",
    "compiz",
    "weston",
    "wayfire",
    "river",
];

/// Probes active Window Manager from running processes, environment, or WSLg.
pub fn detect_wm() -> Option<String> {
    // 1. Check WSLg environment
    if (fs::metadata("/mnt/wslg").is_ok() || std::env::var_os("WSL_DISTRO_NAME").is_some())
        && (std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some())
    {
        return Some("WSLg (Weston)".to_string());
    }

    // 2. Scan `/proc` for active known WM process
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if file_name
                    .to_string_lossy()
                    .chars()
                    .all(|c| c.is_ascii_digit())
                {
                    if let Ok(comm) = fs::read_to_string(path.join("comm")) {
                        let comm_clean = comm.trim().to_lowercase();
                        for &wm in KNOWN_WMS {
                            if comm_clean == wm {
                                let display_name = match wm {
                                    "kwin_wayland" | "kwin_x11" | "kwin" => "KWin",
                                    "mutter" | "gnome-shell" => "Mutter",
                                    "xfwm4" => "Xfwm4",
                                    "muffin" => "Muffin",
                                    "marco" => "Marco",
                                    "sway" => "Sway",
                                    "hyprland" => "Hyprland",
                                    "i3" => "i3",
                                    "bspwm" => "bspwm",
                                    "dwm" => "dwm",
                                    "awesome" => "awesome",
                                    "xmonad" => "xmonad",
                                    "qtile" => "qtile",
                                    "openbox" => "Openbox",
                                    "fluxbox" => "Fluxbox",
                                    "enlightenment" => "Enlightenment",
                                    "compiz" => "Compiz",
                                    "weston" => "Weston",
                                    "wayfire" => "Wayfire",
                                    "river" => "River",
                                    _ => wm,
                                };
                                return Some(display_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Fallback to desktop session environment hints
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        let de_lower = de.to_lowercase();
        if de_lower.contains("gnome") {
            return Some("Mutter".to_string());
        } else if de_lower.contains("kde") {
            return Some("KWin".to_string());
        } else if de_lower.contains("xfce") {
            return Some("Xfwm4".to_string());
        } else if de_lower.contains("cinnamon") {
            return Some("Muffin".to_string());
        } else if de_lower.contains("mate") {
            return Some("Marco".to_string());
        } else if de_lower.contains("sway") {
            return Some("Sway".to_string());
        } else if de_lower.contains("hyprland") {
            return Some("Hyprland".to_string());
        } else if de_lower.contains("i3") {
            return Some("i3".to_string());
        }
    }

    None
}

pub struct WmCollector;

impl Collector for WmCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Wm
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let wm = detect_wm()?;
        Some(ModuleOutput {
            id: ModuleId::Wm,
            label: "WM".to_string(),
            value: wm,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_wm_live() {
        let _ = detect_wm();
    }
}
