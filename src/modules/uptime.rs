use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;
use std::mem::MaybeUninit;

/// Parses total uptime seconds from `/proc/uptime` content.
pub fn parse_uptime(content: &str) -> Option<u64> {
    let first_token = content.split_whitespace().next()?;
    let seconds_f64: f64 = first_token.parse().ok()?;
    if seconds_f64 >= 0.0 {
        Some(seconds_f64 as u64)
    } else {
        None
    }
}

/// Formats total seconds into a readable day/hour/minute representation.
pub fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        let day_label = if days == 1 { "day" } else { "days" };
        let hour_label = if hours == 1 { "hour" } else { "hours" };
        format!(
            "{} {}, {} {}, {} mins",
            days, day_label, hours, hour_label, minutes
        )
    } else if hours > 0 {
        let hour_label = if hours == 1 { "hour" } else { "hours" };
        format!("{} {}, {} mins", hours, hour_label, minutes)
    } else {
        format!("{} mins", minutes)
    }
}

/// Reads system uptime from `/proc/uptime` or `libc::sysinfo`.
pub fn get_uptime() -> Option<u64> {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(secs) = parse_uptime(&content) {
            return Some(secs);
        }
    }

    unsafe {
        let mut info = MaybeUninit::<libc::sysinfo>::uninit();
        if libc::sysinfo(info.as_mut_ptr()) == 0 {
            let info = info.assume_init();
            if info.uptime >= 0 {
                return Some(info.uptime as u64);
            }
        }
    }

    None
}

pub struct UptimeCollector;

impl Collector for UptimeCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Uptime
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let secs = get_uptime()?;
        Some(ModuleOutput {
            id: ModuleId::Uptime,
            label: "Uptime".to_string(),
            value: format_uptime(secs),
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uptime_standard() {
        let fixture = "1978.59 23627.21\n";
        assert_eq!(parse_uptime(fixture), Some(1978));
    }

    #[test]
    fn test_format_uptime_days() {
        let secs = 2 * 86400 + 5 * 3600 + 30 * 60;
        assert_eq!(format_uptime(secs), "2 days, 5 hours, 30 mins");
    }

    #[test]
    fn test_format_uptime_single_day_single_hour() {
        let secs = 86400 + 3600 + 10 * 60;
        assert_eq!(format_uptime(secs), "1 day, 1 hour, 10 mins");
    }

    #[test]
    fn test_format_uptime_hours() {
        let secs = 3 * 3600 + 15 * 60;
        assert_eq!(format_uptime(secs), "3 hours, 15 mins");
    }

    #[test]
    fn test_format_uptime_minutes() {
        let secs = 42 * 60 + 12;
        assert_eq!(format_uptime(secs), "42 mins");
    }
}
