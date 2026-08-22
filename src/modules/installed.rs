use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
#[cfg(not(windows))]
use std::ffi::CString;
#[cfg(not(windows))]
use std::fs;
#[cfg(not(windows))]
use std::mem::MaybeUninit;
use std::time::UNIX_EPOCH;

#[cfg(not(windows))]
#[repr(C)]
#[derive(Default)]
struct StatxTimestamp {
    tv_sec: i64,
    tv_nsec: u32,
    __statx_pad1: i32,
}

#[cfg(not(windows))]
#[repr(C)]
#[derive(Default)]
struct Statx {
    stx_mask: u32,
    stx_blksize: u32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    __statx_pad1: [u16; 1],
    stx_ino: u64,
    stx_size: u64,
    stx_blocks: u64,
    stx_attributes_mask: u64,
    stx_atime: StatxTimestamp,
    stx_btime: StatxTimestamp,
    stx_ctime: StatxTimestamp,
    stx_mtime: StatxTimestamp,
    stx_rdev_major: u32,
    stx_rdev_minor: u32,
    stx_dev_major: u32,
    stx_dev_minor: u32,
    stx_mnt_id: u64,
    __statx_pad2: [u64; 14],
}

#[cfg(not(windows))]
const STATX_BTIME: u32 = 0x00000800;
#[cfg(not(windows))]
const STATX_MTIME: u32 = 0x00000020;
#[cfg(not(windows))]
const STATX_CTIME: u32 = 0x00000040;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallInfo {
    pub timestamp: u64,
    pub formatted: String,
}

/// Direct statx syscall wrapper (Linux >= 4.11).
/// Direct syscall invocation is required because `std::fs::Metadata::created()` is unsupported
/// on older kernels/glibc versions and filesystems lacking explicit btime support.
#[cfg(not(windows))]
fn get_statx_birth_time(path: &str) -> Option<u64> {
    let c_path = CString::new(path).ok()?;
    let mut statx_buf = MaybeUninit::<Statx>::zeroed();
    let res = unsafe {
        libc::syscall(
            libc::SYS_statx,
            libc::AT_FDCWD,
            c_path.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
            STATX_BTIME | STATX_CTIME | STATX_MTIME,
            statx_buf.as_mut_ptr(),
        )
    };
    if res == 0 {
        let buf = unsafe { statx_buf.assume_init() };
        // Check if the underlying filesystem returned valid btime (creation time)
        if buf.stx_mask & STATX_BTIME != 0 && buf.stx_btime.tv_sec > 0 {
            return Some(buf.stx_btime.tv_sec as u64);
        }
        // Fallback to inode ctime (status change time) when btime is unsupported (e.g. ext3, tmpfs)
        if buf.stx_ctime.tv_sec > 0 {
            return Some(buf.stx_ctime.tv_sec as u64);
        }
    }
    None
}

#[cfg(not(windows))]
fn get_metadata_ctime(path: &str) -> Option<u64> {
    let meta = fs::metadata(path).ok()?;
    let created = meta.created().or_else(|_| meta.modified()).ok()?;
    let duration = created.duration_since(UNIX_EPOCH).ok()?;
    Some(duration.as_secs())
}

/// Probes OS installation timestamp via filesystem root birth time and installer logs.
#[cfg(not(windows))]
pub fn detect_install_timestamp() -> Option<u64> {
    // 1. Filesystem root `/` birth time is the primary installation indicator
    if let Some(ts) = get_statx_birth_time("/") {
        // Sanity check: must be after year 2000 (946684800) to filter uninitialized RTC timestamps (e.g. 1970-01-01)
        if ts > 946684800 {
            return Some(ts);
        }
    }

    // 2. Candidate distribution installer logs and earliest created package database files
    let candidate_paths = [
        "/var/log/installer",    // Debian/Ubuntu Ubiquity/Subiquity
        "/var/log/anaconda",     // RHEL/Fedora/CentOS Anaconda
        "/var/log/pacman.log",   // Arch Linux initial pacstrap log
        "/var/lib/dpkg/info",    // Debian base packages
        "/var/lib/pacman/local", // Arch local db
        "/var/lib/rpm",          // RPM database root
        "/etc/apk/world",        // Alpine base package set
        "/etc/machine-id",       // systemd first-boot machine ID
        "/etc/fstab",            // Installer partition table setup
    ];

    for &path in &candidate_paths {
        if let Some(ts) = get_statx_birth_time(path).or_else(|| get_metadata_ctime(path)) {
            if ts > 946684800 {
                return Some(ts);
            }
        }
    }

    None
}

/// Reads OS installation timestamp from Windows registry.
#[cfg(windows)]
pub fn detect_install_timestamp() -> Option<u64> {
    use crate::modules::win_util::ffi;
    let key = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";
    ffi::reg_read_u64(ffi::HKEY_LOCAL_MACHINE, key, "InstallDate")
}

/// Converts UNIX epoch seconds into (year, month 1-12, day 1-31, hour 0-23, minute 0-59, second 0-59).
pub fn epoch_to_datetime(epoch_secs: u64) -> (i32, u8, u8, u8, u8, u8) {
    let secs = epoch_secs % 86400;
    let hour = (secs / 3600) as u8;
    let minute = ((secs % 3600) / 60) as u8;
    let second = (secs % 60) as u8;

    let mut days = (epoch_secs / 86400) as i64;
    days += 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = (days - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y } as i32;
    (year, m as u8, d as u8, hour, minute, second)
}

/// Formats installation timestamp into `DD Mon YYYY, hh:mm AM/PM (X days ago)`
pub fn format_install_date(timestamp: u64, now_sec: u64) -> String {
    let (year, month_num, day, hour, minute, _) = epoch_to_datetime(timestamp);
    let month = match month_num {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    };
    let (h12, ampm) = if hour == 0 {
        (12, "AM")
    } else if hour < 12 {
        (hour, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else {
        (hour - 12, "PM")
    };
    let date_str = format!(
        "{:02} {} {}, {:02}:{:02} {}",
        day, month, year, h12, minute, ampm
    );

    let diff_sec = now_sec.saturating_sub(timestamp);
    let total_days = diff_sec / 86400;

    let relative_str = if total_days == 0 {
        "today".to_string()
    } else if total_days == 1 {
        "1 day ago".to_string()
    } else if total_days < 365 {
        format!("{} days ago", total_days)
    } else {
        let years = total_days / 365;
        let rem_days = total_days % 365;
        let y_label = if years == 1 { "year" } else { "years" };
        if rem_days == 0 {
            format!("{} {} ago", years, y_label)
        } else if rem_days == 1 {
            format!("{} {}, 1 day ago", years, y_label)
        } else {
            format!("{} {}, {} days ago", years, y_label, rem_days)
        }
    };

    format!("{} ({})", date_str, relative_str)
}

pub struct InstalledCollector;

impl Collector for InstalledCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Installed
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let ts = detect_install_timestamp()?;
        let now_sec = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(ts);

        let value = format_install_date(ts, now_sec);
        Some(ModuleOutput {
            id: ModuleId::Installed,
            label: "Installed".to_string(),
            value,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_install_date_today() {
        let now = 1787140000;
        let s = format_install_date(now, now);
        assert!(s.contains("(today)"));
    }

    #[test]
    fn test_format_install_date_days_ago() {
        let now = 1787140000;
        let past = now - (3 * 86400);
        let s = format_install_date(past, now);
        assert!(s.contains("(3 days ago)"));
    }

    #[test]
    fn test_format_install_date_years_ago() {
        let now = 1787140000;
        let past = now - ((365 * 2 + 10) * 86400);
        let s = format_install_date(past, now);
        assert!(s.contains("(2 years, 10 days ago)"));
    }

    #[test]
    fn test_epoch_to_datetime() {
        // 2026-08-19 11:46:40 UTC
        let (y, m, d, h, min, s) = epoch_to_datetime(1787140000);
        assert_eq!(y, 2026);
        assert_eq!(m, 8);
        assert_eq!(d, 19);
        assert_eq!(h, 11);
        assert_eq!(min, 46);
        assert_eq!(s, 40);

        // 1970-01-01 00:00:00 UTC
        let (y, m, d, h, min, s) = epoch_to_datetime(0);
        assert_eq!(y, 1970);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
        assert_eq!(h, 0);
        assert_eq!(min, 0);
        assert_eq!(s, 0);
    }

    #[cfg(not(windows))]
    #[test]
    fn test_detect_install_timestamp_live() {
        let ts = detect_install_timestamp();
        assert!(ts.is_some());
        assert!(ts.unwrap() > 946684800);
    }
}
