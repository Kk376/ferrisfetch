use crate::context::FetchContext;
use crate::modules::kernel::get_uname_info;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use crate::output::color::{bold, RESET};
use crate::output::logo::match_logo;
use std::ffi::CStr;
use std::fs;

/// Retrieves the current username from environment or POSIX passwd.
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

/// Retrieves the system hostname from uname, /proc, or /etc.
pub fn get_hostname() -> String {
    if let Some(uname) = get_uname_info() {
        if !uname.hostname.is_empty() && uname.hostname != "(none)" {
            return uname.hostname;
        }
    }

    if let Ok(host) = fs::read_to_string("/proc/sys/kernel/hostname") {
        let clean = host.trim();
        if !clean.is_empty() && clean != "(none)" {
            return clean.to_string();
        }
    }

    if let Ok(host) = fs::read_to_string("/etc/hostname") {
        let clean = host.trim();
        if !clean.is_empty() && clean != "(none)" {
            return clean.to_string();
        }
    }

    "localhost".to_string()
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
        let divider_len = title_plain.chars().count();
        let divider_plain = "-".repeat(divider_len);

        let custom_rendered = if ctx.enable_color {
            let logo = match_logo(
                ctx.logo_override.as_deref(),
                &ctx.os_info.distro_id,
                &ctx.os_info.distro_like,
            );
            let primary = logo.map(|l| l.primary_color).unwrap_or("\x1b[38;5;208m");

            let user_styled = format!("{}{}{}", primary, bold(&user, true), RESET);
            let host_styled = format!("{}{}{}", primary, bold(&host, true), RESET);
            let line1 = format!("{}@{}", user_styled, host_styled);
            let line2 = divider_plain;
            format!("{}\n{}", line1, line2)
        } else {
            format!("{}\n{}", title_plain, divider_plain)
        };

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
}
