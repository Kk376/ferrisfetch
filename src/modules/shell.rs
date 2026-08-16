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

/// Probes process hierarchy or environment to determine active user shell.
pub fn detect_shell() -> Option<String> {
    let mut current_pid = unsafe { libc::getpid() as u32 };

    // Traverse up to 5 process ancestors
    for _ in 0..5 {
        if let Some(ppid) = get_ppid(current_pid) {
            if ppid <= 1 {
                break;
            }
            if let Some(name) = get_proc_name(ppid) {
                let name_lower = name.to_lowercase();
                for &known in KNOWN_SHELLS {
                    if name_lower == known || name_lower.starts_with(known) {
                        return Some(format_shell_with_version(&name_lower));
                    }
                }
            }
            current_pid = ppid;
        } else {
            break;
        }
    }

    // Fallback to $SHELL environment variable
    if let Ok(shell_path) = std::env::var("SHELL") {
        if let Some(name) = Path::new(&shell_path).file_name() {
            let name_str = name.to_string_lossy().to_lowercase();
            if !name_str.is_empty() {
                return Some(format_shell_with_version(&name_str));
            }
        }
    }

    None
}

fn format_shell_with_version(shell_name: &str) -> String {
    if shell_name.contains("bash") {
        if let Ok(ver) = std::env::var("BASH_VERSION") {
            let clean_ver = ver.split('(').next().unwrap_or(&ver).trim();
            if !clean_ver.is_empty() {
                return format!("bash {}", clean_ver);
            }
        }
    } else if shell_name.contains("zsh") {
        if let Ok(ver) = std::env::var("ZSH_VERSION") {
            let clean_ver = ver.trim();
            if !clean_ver.is_empty() {
                return format!("zsh {}", clean_ver);
            }
        }
    } else if shell_name.contains("fish") {
        if let Ok(ver) = std::env::var("FISH_VERSION") {
            let clean_ver = ver.trim();
            if !clean_ver.is_empty() {
                return format!("fish {}", clean_ver);
            }
        }
    }

    shell_name.to_string()
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
    fn test_format_shell_plain() {
        assert_eq!(format_shell_with_version("sh"), "sh");
    }
}
