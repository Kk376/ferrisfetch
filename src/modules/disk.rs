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
#[allow(clippy::unnecessary_cast)]
pub fn get_disk_usage(path: &str) -> Option<DiskUsage> {
    let c_path = CString::new(path).ok()?;
    unsafe {
        let mut stat = MaybeUninit::<libc::statvfs>::uninit();
        if libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) != 0 {
            return None;
        }
        let stat = stat.assume_init();

        let block_size = if stat.f_frsize > 0 {
            stat.f_frsize as u64
        } else {
            stat.f_bsize as u64
        };

        let total_bytes = (stat.f_blocks as u64).saturating_mul(block_size);
        let free_bytes = (stat.f_bavail as u64).saturating_mul(block_size);
        let used_bytes =
            total_bytes.saturating_sub((stat.f_bfree as u64).saturating_mul(block_size));

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

const IGNORED_FS_TYPES: &[&str] = &[
    "tmpfs",
    "devtmpfs",
    "proc",
    "sysfs",
    "cgroup",
    "cgroup2",
    "overlay",
    "squashfs",
    "tracefs",
    "debugfs",
    "pstore",
    "bpf",
    "fusectl",
    "configfs",
    "binfmt_misc",
    "securityfs",
    "mqueue",
    "hugetlbfs",
    "autofs",
    "ramfs",
    "devpts",
    "nsfs",
    "efivarfs",
    "selinuxfs",
    "fuse.gvfsd-fuse",
    "fuse.portal",
    "erofs",
    "rootfs",
    "sdcardfs",
];

const IGNORED_MOUNT_PREFIXES: &[&str] = &[
    "/mnt/wsl",
    "/mnt/wslg",
    "/usr/lib/wsl/drivers",
    "/init",
    "/dev",
    "/run",
    "/sys",
    "/proc",
    "/var/lib/docker",
    "/var/lib/containers",
    "/var/lib/flatpak",
    "/snap",
    "/apex",
    "/bootstrap-apex",
    "/data/app",
    "/data/user",
    "/data/data",
    "/data/media",
    "/data_mirror",
    "/storage/emulated",
    "/mnt/runtime",
    "/mnt/user",
    "/mnt/installer",
    "/mnt/androidwritable",
    "/mnt/pass_through",
    "/mnt/media_rw",
    "/system",
    "/system_ext",
    "/vendor",
    "/product",
    "/odm",
    "/oem",
    "/metadata",
    "/acct",
    "/config",
    "/linkerconfig",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionEntry {
    pub mount_point: String,
    pub display_name: String,
    pub usage: DiskUsage,
}

/// Enumerates all real physical/virtual mount partitions.
pub fn get_all_disks() -> Vec<PartitionEntry> {
    let mut entries = Vec::new();
    let Ok(content) = std::fs::read_to_string("/proc/mounts") else {
        if let Some(usage) = get_disk_usage("/") {
            entries.push(PartitionEntry {
                mount_point: "/".to_string(),
                display_name: "/".to_string(),
                usage,
            });
        }
        return entries;
    };

    let mut seen_mounts = std::collections::HashSet::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let mount_point = parts[1];
        let fs_type = parts[2];

        if IGNORED_FS_TYPES.contains(&fs_type) {
            continue;
        }

        if IGNORED_MOUNT_PREFIXES
            .iter()
            .any(|prefix| mount_point.starts_with(prefix))
        {
            continue;
        }

        if !seen_mounts.insert(mount_point.to_string()) {
            continue;
        }

        if let Some(usage) = get_disk_usage(mount_point) {
            // In WSL, convert /mnt/c to C, /mnt/d to D
            let display_name = if let Some(wsl_drive) = mount_point.strip_prefix("/mnt/") {
                if wsl_drive.len() == 1 && wsl_drive.chars().next().unwrap().is_ascii_alphabetic() {
                    wsl_drive.to_uppercase()
                } else {
                    mount_point.to_string()
                }
            } else {
                mount_point.to_string()
            };

            entries.push(PartitionEntry {
                mount_point: mount_point.to_string(),
                display_name,
                usage,
            });
        }
    }

    if entries.is_empty() {
        if let Some(usage) = get_disk_usage("/") {
            entries.push(PartitionEntry {
                mount_point: "/".to_string(),
                display_name: "/".to_string(),
                usage,
            });
        }
    }

    // Always sort so root / comes first (Disk0), followed by other partitions
    entries.sort_by(|a, b| {
        if a.mount_point == "/" {
            std::cmp::Ordering::Less
        } else if b.mount_point == "/" {
            std::cmp::Ordering::Greater
        } else {
            a.display_name.cmp(&b.display_name)
        }
    });

    entries
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
            label: "Disk0".to_string(),
            value: format!("(/) {}", format_disk_usage(&usage)),
            custom_rendered: None,
        })
    }

    fn collect_multiple(&self, ctx: &FetchContext) -> Vec<ModuleOutput> {
        if ctx.disk_target_path != "/" {
            if let Some(usage) = get_disk_usage(&ctx.disk_target_path) {
                return vec![ModuleOutput {
                    id: ModuleId::Disk,
                    label: "Disk0".to_string(),
                    value: format!("({}) {}", ctx.disk_target_path, format_disk_usage(&usage)),
                    custom_rendered: None,
                }];
            } else {
                return Vec::new();
            }
        }

        let disks = get_all_disks();
        let mut outputs = Vec::new();

        for (idx, entry) in disks.iter().enumerate() {
            let label = format!("Disk{}", idx);
            let value = format!(
                "({}) {}",
                entry.display_name,
                format_disk_usage(&entry.usage)
            );
            outputs.push(ModuleOutput {
                id: ModuleId::Disk,
                label,
                value,
                custom_rendered: None,
            });
        }

        outputs
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

    #[test]
    fn test_android_mounts_filter() {
        let sample_mounts = r#"
/dev/root / ext4 ro,relatime 0 0
/dev/block/dm-0 /apex/com.android.runtime erofs ro,relatime 0 0
/dev/block/dm-1 /apex/com.android.art@361099999 erofs ro,relatime 0 0
/dev/block/dm-2 /bootstrap-apex/com.android.runtime erofs ro,relatime 0 0
/dev/block/bootdevice/by-name/userdata /data f2fs rw,nosuid,nodev,noatime 0 0
/dev/block/dm-3 /data/app/~~XYZ==/com.google.android.youtube==/base.apk erofs ro,nodev 0 0
/data/media /storage/emulated sdcardfs rw,nosuid,nodev 0 0
/dev/block/vold/public:179,1 /storage/FF70-CD48 vfat rw,dirsync,nosuid,nodev 0 0
/dev/block/bootdevice/by-name/product /product erofs ro,relatime 0 0
/dev/block/bootdevice/by-name/vendor /vendor erofs ro,relatime 0 0
/dev/block/bootdevice/by-name/metadata /metadata f2fs rw,sync 0 0
"#;

        let filtered: Vec<&str> = sample_mounts
            .lines()
            .filter_map(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() < 3 {
                    return None;
                }
                let mp = parts[1];
                let fs = parts[2];
                if IGNORED_FS_TYPES.contains(&fs) {
                    return None;
                }
                if IGNORED_MOUNT_PREFIXES
                    .iter()
                    .any(|prefix| mp.starts_with(prefix))
                {
                    return None;
                }
                Some(mp)
            })
            .collect();

        assert_eq!(filtered, vec!["/", "/data", "/storage/FF70-CD48"]);
    }
}
