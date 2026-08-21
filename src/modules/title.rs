use crate::context::FetchContext;
use crate::modules::kernel::get_uname_info;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use crate::output::color::RESET;
use crate::output::logo::match_logo;
use std::ffi::CStr;
use std::fs;

/// Retrieves the current username from environment or POSIX passwd database.
pub fn get_username() -> String {
    if let Ok(user) = std::env::var("USER") {
        if !user.trim().is_empty() {
            return user.trim().to_string();
        }
    }
    if let Ok(user) = std::env::var("LOGNAME") {
        if !user.trim().is_empty() {
            return user.trim().to_string();
        }
    }

    // Fallback to POSIX user database entry for effective UID
    unsafe {
        let uid = libc::geteuid();
        let pw = libc::getpwuid(uid);
        if !pw.is_null() {
            let name = CStr::from_ptr((*pw).pw_name);
            return name.to_string_lossy().into_owned();
        }
    }

    "user".to_string()
}

/// Retrieves the system hostname from uname nodename, /proc, or /etc.
pub fn get_hostname() -> String {
    // 1. Direct uname nodename field
    if let Some(uname) = get_uname_info() {
        if !uname.hostname.is_empty() && uname.hostname != "(none)" {
            return uname.hostname;
        }
    }

    // 2. Kernel sysctl procfs hostname
    if let Ok(host) = fs::read_to_string("/proc/sys/kernel/hostname") {
        let clean = host.trim();
        if !clean.is_empty() && clean != "(none)" {
            return clean.to_string();
        }
    }

    // 3. Static configuration file fallback
    if let Ok(host) = fs::read_to_string("/etc/hostname") {
        let clean = host.trim();
        if !clean.is_empty() && clean != "(none)" {
            return clean.to_string();
        }
    }

    "localhost".to_string()
}

/// Formats the title line (`user@host`) and matching underline separator.
pub fn format_title(
    user: &str,
    host: &str,
    primary_color: Option<&str>,
    enable_color: bool,
) -> String {
    let title_plain = format!("{}@{}", user, host);
    // Divider length exactly matches printable character count to prevent underline misalignment
    let divider_len = title_plain.chars().count();
    let divider_plain = "-".repeat(divider_len);

    if enable_color {
        let primary = primary_color.unwrap_or("\x1b[38;5;208m");
        let user_styled = format!("{}{}{}{}", crate::output::color::BOLD, primary, user, RESET);
        let host_styled = format!("{}{}{}{}", crate::output::color::BOLD, primary, host, RESET);
        let line1 = format!("{}@{}", user_styled, host_styled);
        format!("{}\n{}", line1, divider_plain)
    } else {
        format!("{}\n{}", title_plain, divider_plain)
    }
}

pub struct TitleCollector;

impl Collector for TitleCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Title
    }

    fn collect(&self, ctx: &FetchContext) -> Option<ModuleOutput> {
        let user = get_username();
        let host = get_hostname();
        let title_plain = format!("{}@{}", user, host);

        let logo = match_logo(
            ctx.logo_override.as_deref(),
            &ctx.os_info.distro_id,
            &ctx.os_info.distro_like,
        );
        let primary = logo.map(|l| l.primary_color);

        let custom_rendered = format_title(&user, &host, primary, ctx.enable_color);

        Some(ModuleOutput {
            id: ModuleId::Title,
            label: String::new(),
            value: title_plain,
            custom_rendered: Some(custom_rendered),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_and_hostname_not_empty() {
        let user = get_username();
        let host = get_hostname();
        assert!(!user.is_empty());
        assert!(!host.is_empty());
    }

    #[test]
    fn test_format_title_plain() {
        let rendered = format_title("ferris", "crab", None, false);
        assert_eq!(rendered, "ferris@crab\n-----------");
    }

    #[test]
    fn test_format_title_long_hostname() {
        let user = "admin";
        let host = "super-long-hostname-node-123.region-east.internal.cloud";
        let rendered = format_title(user, host, None, false);
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].len(), lines[1].len());
        assert_eq!(lines[0], format!("{}@{}", user, host));
        assert_eq!(lines[1], "-".repeat(lines[0].len()));
    }
}
