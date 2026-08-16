use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

const KNOWN_TERMINALS: &[(&str, &str)] = &[
    ("gnome-terminal-server", "GNOME Terminal"),
    ("gnome-terminal", "GNOME Terminal"),
    ("konsole", "Konsole"),
    ("alacritty", "Alacritty"),
    ("kitty", "kitty"),
    ("wezterm-gui", "WezTerm"),
    ("wezterm", "WezTerm"),
    ("foot", "foot"),
    ("xterm", "xterm"),
    ("urxvt", "urxvt"),
    ("rxvt", "rxvt"),
    ("st", "st"),
    ("terminator", "Terminator"),
    ("xfce4-terminal", "XFCE Terminal"),
    ("tilix", "Tilix"),
    ("ghostty", "Ghostty"),
    ("tabby", "Tabby"),
    ("hyper", "Hyper"),
    ("tmux", "tmux"),
];

/// Inspects environment variables and process ancestry to detect terminal emulator.
pub fn detect_terminal() -> Option<String> {
    // 1. Check $TERM_PROGRAM
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let clean_prog = prog.trim();
        if !clean_prog.is_empty() {
            if let Ok(ver) = std::env::var("TERM_PROGRAM_VERSION") {
                let clean_ver = ver.trim();
                if !clean_ver.is_empty() {
                    return Some(format!("{} {}", clean_prog, clean_ver));
                }
            }
            return Some(clean_prog.to_string());
        }
    }

    // 2. Check dedicated terminal environment signatures
    if std::env::var_os("ALACRITTY_LOG").is_some()
        || std::env::var_os("ALACRITTY_WINDOW_ID").is_some()
        || std::env::var_os("ALACRITTY_SOCKET").is_some()
    {
        return Some("Alacritty".to_string());
    }

    if std::env::var_os("KITTY_PID").is_some() || std::env::var_os("KITTY_WINDOW_ID").is_some() {
        return Some("kitty".to_string());
    }

    if let Ok(ver) = std::env::var("KONSOLE_VERSION") {
        let clean = ver.trim();
        if !clean.is_empty() {
            return Some(format!("Konsole {}", clean));
        }
        return Some("Konsole".to_string());
    }

    if std::env::var_os("WT_SESSION").is_some() {
        return Some("Windows Terminal".to_string());
    }

    if std::env::var_os("FOOT_PID").is_some() {
        return Some("foot".to_string());
    }

    if std::env::var_os("TERMINOLOGY").is_some() {
        return Some("Terminology".to_string());
    }

    if let Ok(ver) = std::env::var("XTERM_VERSION") {
        let clean = ver.trim();
        if !clean.is_empty() {
            return Some(format!("xterm {}", clean));
        }
        return Some("xterm".to_string());
    }

    if std::env::var_os("GNOME_TERMINAL_SCREEN").is_some()
        || std::env::var_os("GNOME_TERMINAL_SERVICE").is_some()
    {
        return Some("GNOME Terminal".to_string());
    }

    if std::env::var_os("TILIX_ID").is_some() {
        return Some("Tilix".to_string());
    }

    if std::env::var_os("WEZTERM_PANE").is_some() {
        return Some("WezTerm".to_string());
    }

    // 3. Process ancestry traversal
    let mut current_pid = unsafe { libc::getpid() as u32 };
    for _ in 0..8 {
        let status_path = format!("/proc/{}/status", current_pid);
        let ppid = if let Ok(status) = fs::read_to_string(status_path) {
            status.lines().find_map(|l| {
                l.strip_prefix("PPid:")
                    .and_then(|p| p.trim().parse::<u32>().ok())
            })
        } else {
            None
        };

        if let Some(ppid) = ppid {
            if ppid <= 1 {
                break;
            }

            let comm = fs::read_to_string(format!("/proc/{}/comm", ppid))
                .unwrap_or_default()
                .trim()
                .to_lowercase();

            for &(proc_name, display_name) in KNOWN_TERMINALS {
                if comm == proc_name || comm.contains(proc_name) {
                    return Some(display_name.to_string());
                }
            }

            current_pid = ppid;
        } else {
            break;
        }
    }

    // 4. Fallback to $TERM
    if let Ok(term) = std::env::var("TERM") {
        let clean = term.trim();
        if !clean.is_empty() && clean != "unknown" {
            return Some(clean.to_string());
        }
    }

    None
}

pub struct TerminalCollector;

impl Collector for TerminalCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Terminal
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let term = detect_terminal()?;
        Some(ModuleOutput {
            id: ModuleId::Terminal,
            label: "Terminal".to_string(),
            value: term,
            custom_rendered: None,
        })
    }
}
