use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::ffi::CString;
use std::fs;
use std::mem::MaybeUninit;
use std::time::UNIX_EPOCH;

#[repr(C)]
#[derive(Default)]
struct StatxTimestamp {
    tv_sec: i64,
    tv_nsec: u32,
    __statx_pad1: i32,
}

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

const STATX_BTIME: u32 = 0x00000800;
const STATX_MTIME: u32 = 0x00000020;
const STATX_CTIME: u32 = 0x00000040;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallInfo {
    pub timestamp: u64,
    pub formatted: String,
}

/// Direct statx syscall wrapper (Linux >= 4.11).
/// Direct syscall invocation is required because `std::fs::Metadata::created()` is unsupported
/// on older kernels/glibc versions and filesystems lacking explicit btime support.
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

fn get_metadata_ctime(path: &str) -> Option<u64> {
    let meta = fs::metadata(path).ok()?;
    let created = meta.created().or_else(|_| meta.modified()).ok()?;
    let duration = created.duration_since(UNIX_EPOCH).ok()?;
    Some(duration.as_secs())
}

/// Probes OS installation timestamp via filesystem root birth time and installer logs.
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

/// Formats installation timestamp into `DD Mon YYYY, hh:mm AM/PM (X days ago)`
pub fn format_install_date(timestamp: u64, now_sec: u64) -> String {
    let time_t = timestamp as i64;
    let mut tm = MaybeUninit::<libc::tm>::zeroed();
    let tm_ptr = unsafe { libc::localtime_r(&time_t as *const i64 as *const _, tm.as_mut_ptr()) };

    let date_str = if !tm_ptr.is_null() {
        let tm = unsafe { tm.assume_init() };
        let year = tm.tm_year + 1900;
        let month = match tm.tm_mon {
            0 => "Jan",
            1 => "Feb",
            2 => "Mar",
            3 => "Apr",
            4 => "May",
            5 => "Jun",
            6 => "Jul",
            7 => "Aug",
            8 => "Sep",
            9 => "Oct",
            10 => "Nov",
            _ => "Dec",
        };
        let day = tm.tm_mday;
        let hour = tm.tm_hour;
        let minute = tm.tm_min;
        let (h12, ampm) = if hour == 0 {
            (12, "AM")
        } else if hour < 12 {
            (hour, "AM")
        } else if hour == 12 {
            (12, "PM")
        } else {
            (hour - 12, "PM")
        };
        format!(
            "{:02} {} {}, {:02}:{:02} {}",
            day, month, year, h12, minute, ampm
        )
    } else {
        format!("{}", timestamp)
    };

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
    fn test_detect_install_timestamp_live() {
        let ts = detect_install_timestamp();
        assert!(ts.is_some());
        assert!(ts.unwrap() > 946684800);
    }
}
