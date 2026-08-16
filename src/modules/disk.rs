use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskUsage {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub percentage: u8,
}

/// Queries filesystem storage capacity and usage via POSIX statvfs.
pub fn get_disk_usage(path: &str) -> Option<DiskUsage> {
    let c_path = CString::new(path).ok()?;
    unsafe {
        let mut stat = MaybeUninit::<libc::statvfs>::uninit();
        if libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) != 0 {
            return None;
        }
        let stat = stat.assume_init();

        let block_size = if stat.f_frsize > 0 {
            stat.f_frsize
        } else {
            stat.f_bsize
        };

        let total_bytes = stat.f_blocks.saturating_mul(block_size);
        let free_bytes = stat.f_bavail.saturating_mul(block_size);
        let used_bytes = total_bytes.saturating_sub(stat.f_bfree.saturating_mul(block_size));

        if total_bytes == 0 {
            return None;
        }

        let percentage = ((used_bytes as f64 / total_bytes as f64) * 100.0)
            .round()
            .min(100.0) as u8;

        Some(DiskUsage {
            total_bytes,
            used_bytes,
            free_bytes,
            percentage,
        })
    }
}

/// Formats disk usage into TiB, GiB, or MiB representation.
pub fn format_disk_usage(info: &DiskUsage) -> String {
    const TIB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;

    let total_f = info.total_bytes as f64;
    let used_f = info.used_bytes as f64;

    if total_f >= TIB {
        format!(
            "{:.2} TiB / {:.2} TiB ({}%)",
            used_f / TIB,
            total_f / TIB,
            info.percentage
        )
    } else if total_f >= GIB {
        format!(
            "{:.1} GiB / {:.1} GiB ({}%)",
            used_f / GIB,
            total_f / GIB,
            info.percentage
        )
    } else {
        format!(
            "{:.0} MiB / {:.0} MiB ({}%)",
            used_f / MIB,
            total_f / MIB,
            info.percentage
        )
    }
}

pub struct DiskCollector;

impl Collector for DiskCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Disk
    }

    fn collect(&self, ctx: &FetchContext) -> Option<ModuleOutput> {
        let usage = get_disk_usage(&ctx.disk_target_path)?;
        Some(ModuleOutput {
            id: ModuleId::Disk,
            label: "Disk".to_string(),
            value: format_disk_usage(&usage),
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_disk_usage_gib() {
        let usage = DiskUsage {
            total_bytes: 250 * 1024 * 1024 * 1024,
            used_bytes: 32 * 1024 * 1024 * 1024,
            free_bytes: 218 * 1024 * 1024 * 1024,
            percentage: 13,
        };
        let s = format_disk_usage(&usage);
        assert_eq!(s, "32.0 GiB / 250.0 GiB (13%)");
    }

    #[test]
    fn test_format_disk_usage_tib() {
        let usage = DiskUsage {
            total_bytes: 2 * 1024 * 1024 * 1024 * 1024,
            used_bytes: 1024 * 1024 * 1024 * 1024,
            free_bytes: 1024 * 1024 * 1024 * 1024,
            percentage: 50,
        };
        let s = format_disk_usage(&usage);
        assert_eq!(s, "1.00 TiB / 2.00 TiB (50%)");
    }

    #[test]
    fn test_format_disk_usage_mib() {
        let usage = DiskUsage {
            total_bytes: 500 * 1024 * 1024,
            used_bytes: 100 * 1024 * 1024,
            free_bytes: 400 * 1024 * 1024,
            percentage: 20,
        };
        let s = format_disk_usage(&usage);
        assert_eq!(s, "100 MiB / 500 MiB (20%)");
    }

    #[test]
    fn test_get_disk_usage_root() {
        let usage = get_disk_usage("/");
        assert!(usage.is_some());
        let u = usage.unwrap();
        assert!(u.total_bytes > 0);
    }

    #[test]
    fn test_get_disk_usage_invalid_paths() {
        assert_eq!(get_disk_usage("/nonexistent_path_xyz_987654"), None);
        assert_eq!(get_disk_usage("invalid\0nullbyte"), None);
    }
}
