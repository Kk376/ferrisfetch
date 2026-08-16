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

/// Inspects environment variables to detect terminal emulator.
pub fn detect_terminal_from_env(
    term_program: Option<&str>,
    term_program_version: Option<&str>,
    env_vars: &[(&str, &str)],
    term: Option<&str>,
) -> Option<String> {
    // 1. Check $TERM_PROGRAM
    if let Some(prog) = term_program {
        let clean_prog = prog.trim();
        if !clean_prog.is_empty() {
            if let Some(ver) = term_program_version {
                let clean_ver = ver.trim();
                if !clean_ver.is_empty() {
                    return Some(format!("{} {}", clean_prog, clean_ver));
                }
            }
            return Some(clean_prog.to_string());
        }
    }

    // 2. Check dedicated terminal environment signatures
    let has_env = |var_name: &str| env_vars.iter().any(|&(k, _)| k == var_name);
    let get_env_val = |var_name: &str| {
        env_vars
            .iter()
            .find(|&&(k, _)| k == var_name)
            .map(|&(_, v)| v.trim())
    };

    if has_env("ALACRITTY_LOG") || has_env("ALACRITTY_WINDOW_ID") || has_env("ALACRITTY_SOCKET") {
        return Some("Alacritty".to_string());
    }

    if has_env("KITTY_PID") || has_env("KITTY_WINDOW_ID") {
        return Some("kitty".to_string());
    }

    if let Some(ver) = get_env_val("KONSOLE_VERSION") {
        if !ver.is_empty() {
            return Some(format!("Konsole {}", ver));
        }
        return Some("Konsole".to_string());
    }

    if has_env("WT_SESSION") {
        return Some("Windows Terminal".to_string());
    }

    if has_env("FOOT_PID") {
        return Some("foot".to_string());
    }

    if has_env("TERMINOLOGY") {
        return Some("Terminology".to_string());
    }

    if let Some(ver) = get_env_val("XTERM_VERSION") {
        if !ver.is_empty() {
            return Some(format!("xterm {}", ver));
        }
        return Some("xterm".to_string());
    }

    if has_env("GNOME_TERMINAL_SCREEN") || has_env("GNOME_TERMINAL_SERVICE") {
        return Some("GNOME Terminal".to_string());
    }

    if has_env("TILIX_ID") {
        return Some("Tilix".to_string());
    }

    if has_env("WEZTERM_PANE") {
        return Some("WezTerm".to_string());
    }

    // 3. Fallback to $TERM
    if let Some(t) = term {
        let clean = t.trim();
        if !clean.is_empty() && clean != "unknown" && clean != "dumb" {
            return Some(clean.to_string());
        }
    }

    None
}

pub fn match_terminal_proc(comm: &str) -> Option<&'static str> {
    for &(proc_name, display_name) in KNOWN_TERMINALS {
        let is_match = if proc_name == "st" {
            comm == "st" || comm == "stterm" || comm.starts_with("st-")
        } else {
            comm == proc_name || comm.starts_with(&format!("{}-", proc_name))
        };
        if is_match {
            return Some(display_name);
        }
    }
    None
}

/// Inspects environment variables and process ancestry to detect terminal emulator.
pub fn detect_terminal() -> Option<String> {
    let term_prog = std::env::var("TERM_PROGRAM").ok();
    let term_prog_ver = std::env::var("TERM_PROGRAM_VERSION").ok();
    let term_val = std::env::var("TERM").ok();

    // Check environment first
    let env_signatures = [
        "ALACRITTY_LOG",
        "ALACRITTY_WINDOW_ID",
        "ALACRITTY_SOCKET",
        "KITTY_PID",
        "KITTY_WINDOW_ID",
        "KONSOLE_VERSION",
        "WT_SESSION",
        "FOOT_PID",
        "TERMINOLOGY",
        "XTERM_VERSION",
        "GNOME_TERMINAL_SCREEN",
        "GNOME_TERMINAL_SERVICE",
        "TILIX_ID",
        "WEZTERM_PANE",
    ];

    let mut present_vars = Vec::new();
    for &sig in &env_signatures {
        if let Ok(val) = std::env::var(sig) {
            present_vars.push((sig, val));
        }
    }
    let ref_vars: Vec<(&str, &str)> = present_vars.iter().map(|(k, v)| (*k, v.as_str())).collect();

    if let Some(term) = detect_terminal_from_env(
        term_prog.as_deref(),
        term_prog_ver.as_deref(),
        &ref_vars,
        None, // defer $TERM fallback until process ancestry checked
    ) {
        return Some(term);
    }

    // Process ancestry traversal
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

            if let Some(display_name) = match_terminal_proc(&comm) {
                return Some(display_name.to_string());
            }

            current_pid = ppid;
        } else {
            break;
        }
    }

    // Fallback to $TERM
    if let Some(term) = term_val {
        let clean = term.trim();
        if !clean.is_empty() && clean != "unknown" && clean != "dumb" {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_terminal_from_env_term_program() {
        let res =
            detect_terminal_from_env(Some("WezTerm"), Some("20240203-110809-5046fc22"), &[], None);
        assert_eq!(res, Some("WezTerm 20240203-110809-5046fc22".to_string()));
    }

    #[test]
    fn test_detect_terminal_from_env_alacritty() {
        let envs = [("ALACRITTY_SOCKET", "/run/user/1000/alacritty.sock")];
        let res = detect_terminal_from_env(None, None, &envs, None);
        assert_eq!(res, Some("Alacritty".to_string()));
    }

    #[test]
    fn test_detect_terminal_from_env_kitty() {
        let envs = [("KITTY_PID", "12345")];
        let res = detect_terminal_from_env(None, None, &envs, None);
        assert_eq!(res, Some("kitty".to_string()));
    }

    #[test]
    fn test_detect_terminal_from_env_konsole() {
        let envs = [("KONSOLE_VERSION", "230805")];
        let res = detect_terminal_from_env(None, None, &envs, None);
        assert_eq!(res, Some("Konsole 230805".to_string()));
    }

    #[test]
    fn test_detect_terminal_from_env_fallback_term() {
        let res = detect_terminal_from_env(None, None, &[], Some("xterm-256color"));
        assert_eq!(res, Some("xterm-256color".to_string()));

        let res_unknown = detect_terminal_from_env(None, None, &[], Some("unknown"));
        assert_eq!(res_unknown, None);

        let res_none = detect_terminal_from_env(None, None, &[], None);
        assert_eq!(res_none, None);
    }

    #[test]
    fn test_match_terminal_proc() {
        assert_eq!(match_terminal_proc("st"), Some("st"));
        assert_eq!(match_terminal_proc("stterm"), Some("st"));
        assert_eq!(match_terminal_proc("st-256color"), Some("st"));
        assert_eq!(match_terminal_proc("alacritty"), Some("Alacritty"));
        assert_eq!(match_terminal_proc("kitty"), Some("kitty"));
        assert_eq!(
            match_terminal_proc("gnome-terminal-server"),
            Some("GNOME Terminal")
        );

        // Ensure false-positive substrings do not match
        assert_eq!(match_terminal_proc("systemd"), None);
        assert_eq!(match_terminal_proc("starship"), None);
        assert_eq!(match_terminal_proc("strace"), None);
        assert_eq!(match_terminal_proc("install"), None);
        assert_eq!(match_terminal_proc("gst-plugin"), None);
    }
}
