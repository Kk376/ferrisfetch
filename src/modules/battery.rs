use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
    pub capacity: u8,
    pub status: String,
}

/// Probes physical battery capacity and charging status from sysfs, ignoring WSL/Hyper-V virtual batteries.
pub fn detect_battery() -> Option<BatteryInfo> {
    let power_supply_dir = "/sys/class/power_supply";
    let entries = fs::read_dir(power_supply_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = path.file_name()?.to_string_lossy().to_lowercase();

        if !file_name.starts_with("bat") && !file_name.starts_with("battery") {
            continue;
        }

        // Check model_name and manufacturer to filter out Hyper-V virtual battery in WSL
        let model = fs::read_to_string(path.join("model_name"))
            .unwrap_or_default()
            .to_lowercase();
        let manufacturer = fs::read_to_string(path.join("manufacturer"))
            .unwrap_or_default()
            .to_lowercase();

        if model.contains("hyper-v")
            || model.contains("virtual battery")
            || model.contains("virtual")
            || (manufacturer.contains("microsoft") && model.is_empty())
        {
            continue;
        }

        // Read capacity
        let capacity_str = fs::read_to_string(path.join("capacity")).ok()?;
        let capacity = capacity_str.trim().parse::<u8>().ok()?;

        // Read status (Charging, Discharging, Full, Not charging)
        let status = fs::read_to_string(path.join("status"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

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
    fn test_battery_exclusion_hyperv() {
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

        let model = fs::read_to_string(bat_dir.join("model_name"))
            .unwrap()
            .to_lowercase();
        assert!(model.contains("hyper-v") || model.contains("virtual"));
    }
}
