use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub used_kb: u64,
    pub percent: u64,
}

/// Parses `/proc/meminfo` to calculate total and active used RAM.
pub fn parse_meminfo(content: &str) -> Option<MemoryInfo> {
    let mut mem_total: Option<u64> = None;
    let mut mem_available: Option<u64> = None;
    let mut mem_free: Option<u64> = None;
    let mut buffers: Option<u64> = None;
    let mut cached: Option<u64> = None;
    let mut sreclaimable: Option<u64> = None;
    let mut shmem: Option<u64> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((k, v)) = trimmed.split_once(':') {
            let key = k.trim();
            let val_str = v.split_whitespace().next().unwrap_or("");
            let val: u64 = val_str.parse().unwrap_or(0);

            match key {
                "MemTotal" => mem_total = Some(val),
                "MemAvailable" => mem_available = Some(val),
                "MemFree" => mem_free = Some(val),
                "Buffers" => buffers = Some(val),
                "Cached" => cached = Some(val),
                "SReclaimable" => sreclaimable = Some(val),
                "Shmem" => shmem = Some(val),
                _ => {}
            }
        }
    }

    let total = mem_total?;
    if total == 0 {
        return None;
    }

    let used = if let Some(avail) = mem_available {
        total.saturating_sub(avail)
    } else {
        let free = mem_free.unwrap_or(0);
        let buf = buffers.unwrap_or(0);
        let cach = cached.unwrap_or(0);
        let srec = sreclaimable.unwrap_or(0);
        let shm = shmem.unwrap_or(0);

        let non_used = free + buf + cach + srec;
        total.saturating_sub(non_used) + shm
    };

    let percent = ((used as f64 / total as f64) * 100.0).round() as u64;

    Some(MemoryInfo {
        total_kb: total,
        used_kb: used,
        percent,
    })
}

/// Formats memory in GiB or MiB.
pub fn format_memory(info: &MemoryInfo) -> String {
    let one_gib_kb = 1024.0 * 1024.0;
    if info.total_kb as f64 >= one_gib_kb {
        let used_gib = info.used_kb as f64 / one_gib_kb;
        let total_gib = info.total_kb as f64 / one_gib_kb;
        format!(
            "{:.2} GiB / {:.2} GiB ({}%)",
            used_gib, total_gib, info.percent
        )
    } else {
        let used_mib = info.used_kb as f64 / 1024.0;
        let total_mib = info.total_kb as f64 / 1024.0;
        format!(
            "{:.0} MiB / {:.0} MiB ({}%)",
            used_mib, total_mib, info.percent
        )
    }
}

pub fn get_memory_info() -> Option<MemoryInfo> {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        return parse_meminfo(&content);
    }
    None
}

pub struct MemoryCollector;

impl Collector for MemoryCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Memory
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let mem = get_memory_info()?;
        Some(ModuleOutput {
            id: ModuleId::Memory,
            label: "Memory".to_string(),
            value: format_memory(&mem),
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meminfo_standard() {
        let fixture = r#"
MemTotal:       16281600 kB
MemFree:         4385100 kB
MemAvailable:   11550000 kB
Buffers:          345600 kB
Cached:          7450000 kB
"#;
        let info = parse_meminfo(fixture).unwrap();
        assert_eq!(info.total_kb, 16281600);
        assert_eq!(info.used_kb, 16281600 - 11550000);
        assert_eq!(info.percent, 29);
        let formatted = format_memory(&info);
        assert!(formatted.contains("GiB"));
        assert!(formatted.contains("29%"));
    }

    #[test]
    fn test_parse_meminfo_under_1gib() {
        let fixture = r#"
MemTotal:         512000 kB
MemFree:          100000 kB
MemAvailable:     300000 kB
"#;
        let info = parse_meminfo(fixture).unwrap();
        let formatted = format_memory(&info);
        assert!(formatted.contains("MiB"));
    }
}
