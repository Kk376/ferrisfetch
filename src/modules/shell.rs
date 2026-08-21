use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::path::Path;

/// Retrieves the parent process ID from `/proc/<pid>/status`.
fn get_ppid(pid: u32) -> Option<u32> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("PPid:") {
            return rest.trim().parse::<u32>().ok();
        }
    }
    None
}

/// Retrieves the process command name from `/proc/<pid>/comm` or `/proc/<pid>/exe`.
fn get_proc_name(pid: u32) -> Option<String> {
    let exe_path = format!("/proc/{}/exe", pid);
    if let Ok(target) = fs::read_link(exe_path) {
        if let Some(file_name) = target.file_name() {
            return Some(file_name.to_string_lossy().into_owned());
        }
    }

    let comm_path = format!("/proc/{}/comm", pid);
    if let Ok(comm) = fs::read_to_string(comm_path) {
        let clean = comm.trim().to_string();
        if !clean.is_empty() {
            return Some(clean);
        }
    }

    None
}

const KNOWN_SHELLS: &[&str] = &[
    "bash", "zsh", "fish", "sh", "dash", "ksh", "csh", "tcsh", "nu", "ion", "elvish", "pwsh",
];

/// Extracts and normalizes the shell name from a path or process command.
pub fn extract_shell_name(path: &str) -> String {
    let clean = path.trim();
    if clean.is_empty() {
        return String::new();
    }

    let file_name = Path::new(clean)
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| clean.into());

    // Login shells prepend a leading hyphen in argv[0] (e.g. "-bash" or "-zsh" per POSIX exec/login convention)
    file_name.trim_start_matches('-').to_lowercase()
}

/// Formats shell name with optional version information.
pub fn format_shell_name_version(
    shell_name: &str,
    bash_ver: Option<&str>,
    zsh_ver: Option<&str>,
    fish_ver: Option<&str>,
) -> String {
    let clean_name = shell_name.trim();

    if clean_name.contains("bash") {
        if let Some(ver) = bash_ver {
            let clean_ver = ver.split('(').next().unwrap_or(ver).trim();
            if !clean_ver.is_empty() {
                return format!("bash {}", clean_ver);
            }
        }
    } else if clean_name.contains("zsh") {
        if let Some(ver) = zsh_ver {
            let clean_ver = ver.trim();
            if !clean_ver.is_empty() {
                return format!("zsh {}", clean_ver);
            }
        }
    } else if clean_name.contains("fish") {
        if let Some(ver) = fish_ver {
            let clean_ver = ver.trim();
            if !clean_ver.is_empty() {
                return format!("fish {}", clean_ver);
            }
        }
    }

    clean_name.to_string()
}

fn get_shell_cli_version(shell_name: &str) -> Option<String> {
    let output = std::process::Command::new(shell_name)
        .arg("--version")
        .output()
        .ok()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        for word in text.split_whitespace() {
            let clean = word.trim_matches(',').trim_matches('(').trim_matches(')');
            if clean
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
                && clean.contains('.')
            {
                let ver = clean.split('(').next().unwrap_or(clean);
                return Some(ver.to_string());
            }
        }
    }
    None
}

fn format_shell_with_version(shell_name: &str) -> String {
    // Fast-path: query shell version environment variables before spawning subprocesses
    let mut bash_ver = std::env::var("BASH_VERSION").ok();
    let mut zsh_ver = std::env::var("ZSH_VERSION").ok();
    let mut fish_ver = std::env::var("FISH_VERSION").ok();

    if bash_ver.is_none() && shell_name.contains("bash") {
        bash_ver = get_shell_cli_version("bash");
    }
    if zsh_ver.is_none() && shell_name.contains("zsh") {
        zsh_ver = get_shell_cli_version("zsh");
    }
    if fish_ver.is_none() && shell_name.contains("fish") {
        fish_ver = get_shell_cli_version("fish");
    }

    let res = format_shell_name_version(
        shell_name,
        bash_ver.as_deref(),
        zsh_ver.as_deref(),
        fish_ver.as_deref(),
    );

    if res == shell_name {
        if let Some(cli_ver) = get_shell_cli_version(shell_name) {
            return format!("{} {}", shell_name, cli_ver);
        }
    }

    res
}

/// Checks if a process name matches a known shell or valid versioned shell binary name.
/// Prevents false matches against unrelated daemon names (e.g. `shadow`, `shared-mime`, `shark`).
pub fn is_known_shell(name_clean: &str) -> bool {
    for &known in KNOWN_SHELLS {
        if name_clean == known {
            return true;
        }
        if let Some(rest) = name_clean.strip_prefix(known) {
            if rest.starts_with('-')
                || rest.starts_with('.')
                || (!rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()))
            {
                return true;
            }
        }
    }
    false
}

/// Probes process hierarchy or environment to determine active user shell.
pub fn detect_shell() -> Option<String> {
    let mut current_pid = unsafe { libc::getpid() as u32 };

    // Traverse process parent chain (up to 5 ancestors) to identify the interactive shell that invoked ferrisfetch
    for _ in 0..5 {
        if let Some(ppid) = get_ppid(current_pid) {
            if ppid <= 1 {
                break;
            }
            if let Some(name) = get_proc_name(ppid) {
                let name_clean = extract_shell_name(&name);
                if is_known_shell(&name_clean) {
                    return Some(format_shell_with_version(&name_clean));
                }
            }
            current_pid = ppid;
        } else {
            break;
        }
    }

    // Fallback to $SHELL environment variable if parent process tree is masked or in a container
    if let Ok(shell_path) = std::env::var("SHELL") {
        let name_clean = extract_shell_name(&shell_path);
        if !name_clean.is_empty() {
            return Some(format_shell_with_version(&name_clean));
        }
    }

    None
}

pub struct ShellCollector;

impl Collector for ShellCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Shell
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let shell = detect_shell()?;
        Some(ModuleOutput {
            id: ModuleId::Shell,
            label: "Shell".to_string(),
            value: shell,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_shell_name() {
        assert_eq!(extract_shell_name("/usr/bin/bash"), "bash");
        assert_eq!(extract_shell_name("/bin/-zsh"), "zsh");
        assert_eq!(extract_shell_name("/opt/homebrew/bin/fish"), "fish");
        assert_eq!(extract_shell_name("/usr/bin/nu"), "nu");
        assert_eq!(extract_shell_name("sh"), "sh");
        assert_eq!(extract_shell_name(""), "");
    }

    #[test]
    fn test_format_shell_name_version() {
        assert_eq!(
            format_shell_name_version("bash", Some("5.2.15(1)-release"), None, None),
            "bash 5.2.15"
        );
        assert_eq!(
            format_shell_name_version("zsh", None, Some("5.9"), None),
            "zsh 5.9"
        );
        assert_eq!(
            format_shell_name_version("fish", None, None, Some("3.7.0")),
            "fish 3.7.0"
        );
        assert_eq!(
            format_shell_name_version("custom_shell", None, None, None),
            "custom_shell"
        );
    }

    #[test]
    fn test_is_known_shell() {
        assert!(is_known_shell("bash"));
        assert!(is_known_shell("zsh"));
        assert!(is_known_shell("fish"));
        assert!(is_known_shell("nu"));
        assert!(is_known_shell("sh"));
        assert!(is_known_shell("bash-5.2"));
        assert!(is_known_shell("sh4"));

        // Reject non-shell prefixes
        assert!(!is_known_shell("shadow"));
        assert!(!is_known_shell("shared-mime"));
        assert!(!is_known_shell("nuget"));
        assert!(!is_known_shell("shark"));
    }
}
