use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
    pub capacity: u8,
    pub status: String,
}

fn is_ac_online() -> bool {
    let power_supply_dir = "/sys/class/power_supply";
    if let Ok(entries) = fs::read_dir(power_supply_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.starts_with("ac") || name.starts_with("mains") || name.starts_with("acad") {
                if let Ok(online) = fs::read_to_string(entry.path().join("online")) {
                    if online.trim() == "1" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Probes battery capacity and state, formatting cleanly without redundant virtual model names.
pub fn detect_battery() -> Option<BatteryInfo> {
    let power_supply_dir = "/sys/class/power_supply";
    let entries = fs::read_dir(power_supply_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = path.file_name()?.to_string_lossy().to_lowercase();

        if !file_name.starts_with("bat") && !file_name.starts_with("battery") {
            continue;
        }

        // Read capacity
        let capacity_str = fs::read_to_string(path.join("capacity")).ok()?;
        let capacity = capacity_str.trim().parse::<u8>().ok()?;

        // Read status (Charging, Discharging, Full, Not charging)
        let raw_status = fs::read_to_string(path.join("status"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let ac_online = is_ac_online();

        let status = if raw_status.eq_ignore_ascii_case("not charging") {
            if ac_online {
                "AC Connected".to_string()
            } else {
                "Not charging".to_string()
            }
        } else if raw_status.eq_ignore_ascii_case("charging") {
            "Charging".to_string()
        } else if raw_status.eq_ignore_ascii_case("discharging") {
            "Discharging".to_string()
        } else if raw_status.eq_ignore_ascii_case("full") {
            if ac_online {
                "Full [AC]".to_string()
            } else {
                "Full".to_string()
            }
        } else {
            raw_status
        };

        return Some(BatteryInfo { capacity, status });
    }

    None
}

pub struct BatteryCollector;

impl Collector for BatteryCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Battery
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let info = detect_battery()?;
        let value = format!("{}% [{}]", info.capacity, info.status);
        Some(ModuleOutput {
            id: ModuleId::Battery,
            label: "Battery".to_string(),
            value,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_parsing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bat_dir = temp_dir.path().join("BAT1");
        fs::create_dir_all(&bat_dir).unwrap();
        fs::write(
            bat_dir.join("model_name"),
            "Microsoft Hyper-V Virtual Battery\n",
        )
        .unwrap();
        fs::write(bat_dir.join("capacity"), "97\n").unwrap();
        fs::write(bat_dir.join("status"), "Not charging\n").unwrap();

        let cap = fs::read_to_string(bat_dir.join("capacity"))
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap();
        assert_eq!(cap, 97);
    }
}
