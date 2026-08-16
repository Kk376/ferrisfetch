use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub sockets: usize,
}

/// Cleans redundant marketing and frequency tokens from raw CPU model strings.
pub fn clean_cpu_model(raw: &str) -> String {
    let mut cleaned = raw
        .replace("(R)", "")
        .replace("(TM)", "")
        .replace("(tm)", "")
        .replace("CPU", "")
        .replace("Processor", "")
        .replace("Dual-Core", "")
        .replace("Quad-Core", "")
        .replace("Six-Core", "")
        .replace("Eight-Core", "")
        .replace("12-Core", "")
        .replace("16-Core", "")
        .replace("24-Core", "")
        .replace("32-Core", "")
        .replace("64-Core", "")
        .replace("128-Core", "");

    // Strip clock speed patterns like "@ 2.60GHz"
    if let Some(idx) = cleaned.find('@') {
        cleaned = cleaned[..idx].to_string();
    }

    let tokens: Vec<&str> = cleaned.split_whitespace().collect();
    tokens.join(" ").trim_end_matches(',').trim().to_string()
}

/// Parses `/proc/cpuinfo` into model, logical core count, and physical socket count.
pub fn parse_cpu_info(content: &str) -> Option<CpuInfo> {
    if content.trim().is_empty() {
        return None;
    }

    let mut model_name: Option<String> = None;
    let mut processor_count = 0;
    let mut physical_ids = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_lowercase();
            let val = v.trim();

            match key.as_str() {
                "model name" | "model_name" | "hardware" | "cpu model" => {
                    if model_name.is_none() && !val.is_empty() {
                        model_name = Some(val.to_string());
                    }
                }
                "cpu" => {
                    // PowerPC / POWER architectures (e.g. "cpu : POWER9, altivec supported")
                    if model_name.is_none()
                        && !val.is_empty()
                        && val.chars().any(|c| c.is_alphabetic())
                    {
                        model_name = Some(val.to_string());
                    }
                }
                "processor" => {
                    if val.chars().all(|c| c.is_ascii_digit()) {
                        processor_count += 1;
                    } else if model_name.is_none() && !val.is_empty() {
                        model_name = Some(val.to_string());
                    }
                }
                "model" => {
                    // On ARM/devicetree, 'Model' is the hardware board/CPU name (contains non-digits)
                    if model_name.is_none()
                        && !val.is_empty()
                        && val.chars().any(|c| c.is_alphabetic())
                    {
                        model_name = Some(val.to_string());
                    }
                }
                "physical id" | "physical_id" => {
                    if let Ok(id) = val.parse::<usize>() {
                        physical_ids.insert(id);
                    }
                }
                _ => {}
            }
        }
    }

    if model_name.is_none() && processor_count == 0 {
        return None;
    }

    let model = model_name.unwrap_or_else(|| "Unknown CPU".to_string());
    let cores = if processor_count > 0 {
        processor_count
    } else {
        unsafe {
            let n = libc::sysconf(libc::_SC_NPROCESSORS_ONLN);
            if n > 0 {
                n as usize
            } else {
                1
            }
        }
    };

    let sockets = if !physical_ids.is_empty() {
        physical_ids.len()
    } else {
        1
    };

    Some(CpuInfo {
        model,
        cores,
        sockets,
    })
}

pub fn get_cpu_info() -> Option<CpuInfo> {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        return parse_cpu_info(&content);
    }
    None
}

pub struct CpuCollector;

impl Collector for CpuCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Cpu
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let cpu = get_cpu_info()?;
        let cleaned = clean_cpu_model(&cpu.model);

        let value = if cpu.sockets > 1 {
            format!("{} ({} sockets, {} cores)", cleaned, cpu.sockets, cpu.cores)
        } else if cpu.cores > 0 {
            format!("{} ({})", cleaned, cpu.cores)
        } else {
            cleaned
        };

        Some(ModuleOutput {
            id: ModuleId::Cpu,
            label: "CPU".to_string(),
            value,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_cpu_model_intel() {
        let raw = "Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz";
        assert_eq!(clean_cpu_model(raw), "Intel Core i7-10750H");
    }

    #[test]
    fn test_clean_cpu_model_amd() {
        let raw = "AMD Ryzen 5 7535HS with Radeon Graphics";
        assert_eq!(
            clean_cpu_model(raw),
            "AMD Ryzen 5 7535HS with Radeon Graphics"
        );
    }

    #[test]
    fn test_parse_cpu_info_standard() {
        let fixture = r#"
processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model name	: Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz
physical id	: 0

processor	: 1
vendor_id	: GenuineIntel
cpu family	: 6
model name	: Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz
physical id	: 0
"#;
        let info = parse_cpu_info(fixture).unwrap();
        assert_eq!(info.cores, 2);
        assert_eq!(info.sockets, 1);
        assert_eq!(clean_cpu_model(&info.model), "Intel Core i7-10750H");
    }

    #[test]
    fn test_parse_cpu_info_empty() {
        assert_eq!(parse_cpu_info(""), None);
        assert_eq!(parse_cpu_info("   \n\n  \t "), None);
    }

    #[test]
    fn test_parse_cpu_info_malformed() {
        let malformed = "some_random_key: some_value\nanother_line\n";
        assert_eq!(parse_cpu_info(malformed), None);
    }

    #[test]
    fn test_parse_cpu_info_dual_socket() {
        let fixture = r#"
processor	: 0
model name	: Intel(R) Xeon(R) Gold 6248R CPU @ 3.00GHz
physical id	: 0

processor	: 1
model name	: Intel(R) Xeon(R) Gold 6248R CPU @ 3.00GHz
physical id	: 1
"#;
        let info = parse_cpu_info(fixture).unwrap();
        assert_eq!(info.cores, 2);
        assert_eq!(info.sockets, 2);
        assert_eq!(clean_cpu_model(&info.model), "Intel Xeon Gold 6248R");
    }

    #[test]
    fn test_clean_cpu_model_multi_core_tokens() {
        let raw = "AMD EPYC 7763 64-Core Processor";
        assert_eq!(clean_cpu_model(raw), "AMD EPYC 7763");
    }
}
