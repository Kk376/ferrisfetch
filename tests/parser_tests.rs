use ferrisfetch::modules::cpu::{clean_cpu_model, parse_cpu_info};
use ferrisfetch::modules::memory::{format_memory, parse_meminfo};
use ferrisfetch::modules::os::parse_os_release;
use ferrisfetch::modules::packages::parse_dpkg_status;
use ferrisfetch::modules::uptime::{format_uptime, parse_uptime};
use ferrisfetch::output::formatter::visible_width;
use ferrisfetch::output::logo::match_logo;

#[test]
fn test_fixture_ubuntu_24_04() {
    let content = include_str!("fixtures/os_release/ubuntu_24_04.txt");
    let info = parse_os_release(content);
    assert_eq!(info.display_name, "Ubuntu 24.04 LTS");
    assert_eq!(info.distro_id, "ubuntu");
    assert_eq!(info.distro_like, vec!["debian"]);
}

#[test]
fn test_fixture_arch_rolling() {
    let content = include_str!("fixtures/os_release/arch_rolling.txt");
    let info = parse_os_release(content);
    assert_eq!(info.display_name, "Arch Linux");
    assert_eq!(info.distro_id, "arch");
    assert!(info.distro_like.is_empty());
}

#[test]
fn test_fixture_fedora_40() {
    let content = include_str!("fixtures/os_release/fedora_40.txt");
    let info = parse_os_release(content);
    assert_eq!(info.display_name, "Fedora Linux 40 (Workstation Edition)");
    assert_eq!(info.distro_id, "fedora");
}

#[test]
fn test_fixture_rocky_9() {
    let content = include_str!("fixtures/os_release/rocky_9.txt");
    let info = parse_os_release(content);
    assert_eq!(info.display_name, "Rocky Linux 9.4 (Blue Onyx)");
    assert_eq!(info.distro_id, "rocky");
    assert_eq!(info.distro_like, vec!["rhel", "centos", "fedora"]);
}

#[test]
fn test_fixture_cpu_intel() {
    let content = include_str!("fixtures/cpuinfo/intel_i7_10750h.txt");
    let info = parse_cpu_info(content).expect("Failed to parse intel cpuinfo");
    assert_eq!(info.cores, 2); // 2 processor blocks in fixture
    assert_eq!(info.sockets, 1);
    assert_eq!(clean_cpu_model(&info.model), "Intel Core i7-10750H");
}

#[test]
fn test_fixture_cpu_amd() {
    let content = include_str!("fixtures/cpuinfo/amd_ryzen_7535hs.txt");
    let info = parse_cpu_info(content).expect("Failed to parse amd cpuinfo");
    assert_eq!(
        clean_cpu_model(&info.model),
        "AMD Ryzen 5 7535HS with Radeon Graphics"
    );
}

#[test]
fn test_fixture_cpu_arm_raspberry_pi() {
    let content = include_str!("fixtures/cpuinfo/arm64_raspberry_pi.txt");
    let info = parse_cpu_info(content).expect("Failed to parse rpi cpuinfo");
    assert_eq!(info.cores, 2);
    assert_eq!(info.model, "BCM2835");
}

#[test]
fn test_fixture_meminfo_16gb() {
    let content = include_str!("fixtures/meminfo/standard_16gb.txt");
    let info = parse_meminfo(content).expect("Failed to parse meminfo");
    assert_eq!(info.total_kb, 16281600);
    assert_eq!(info.used_kb, 16281600 - 11550000);
    assert_eq!(info.percent, 29);
    let s = format_memory(&info);
    assert_eq!(s, "4.51 GiB / 15.53 GiB (29%)");
}

#[test]
fn test_fixture_uptime_standard() {
    let content = include_str!("fixtures/uptime/standard.txt");
    let secs = parse_uptime(content).expect("Failed to parse uptime");
    assert_eq!(secs, 1978);
    let s = format_uptime(secs);
    assert_eq!(s, "32 mins");
}

#[test]
fn test_fixture_dpkg_status() {
    let content = include_str!("fixtures/dpkg/status_sample.txt");
    let count = parse_dpkg_status(content);
    assert_eq!(count, 3); // bash (installed), libc6 (installed), coreutils (installed)
}

#[test]
fn test_visible_width_calculation() {
    assert_eq!(visible_width("\x1b[38;5;196mDebian\x1b[0m"), 6);
    assert_eq!(visible_width("Plain String"), 12);
}

#[test]
fn test_logo_resolution() {
    let debian = match_logo(None, "debian", &[]).unwrap();
    assert_eq!(debian.name, "debian");

    let rhel_fallback = match_logo(None, "alma", &["rhel".to_string()]).unwrap();
    assert_eq!(rhel_fallback.name, "almalinux");
}
