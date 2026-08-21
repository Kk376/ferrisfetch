use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::ffi::CStr;
use std::net::Ipv4Addr;

/// Retrieves the primary local IPv4 address using standard POSIX getifaddrs without subprocesses.
pub fn detect_local_ip() -> Option<String> {
    unsafe {
        let mut ifaddrs: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifaddrs) != 0 || ifaddrs.is_null() {
            return None;
        }

        let mut primary_ip: Option<String> = None;
        let mut fallback_ip: Option<String> = None;

        let mut curr = ifaddrs;
        while !curr.is_null() {
            let ifa = *curr;
            if !ifa.ifa_addr.is_null()
                && (*ifa.ifa_addr).sa_family == libc::AF_INET as libc::sa_family_t
            {
                let name = CStr::from_ptr(ifa.ifa_name).to_string_lossy();
                let flags = ifa.ifa_flags as i32;

                let is_up = (flags & libc::IFF_UP) != 0;
                let is_loopback = (flags & libc::IFF_LOOPBACK) != 0;

                if is_up && !is_loopback {
                    let sin = ifa.ifa_addr as *const libc::sockaddr_in;
                    let ip_raw = u32::from_be((*sin).sin_addr.s_addr);
                    let ip = Ipv4Addr::from(ip_raw);

                    // Skip loopback (127.0.0.1) and non-routable link-local APIPA (169.254.0.0/16)
                    if !ip.is_loopback() && !ip.is_link_local() {
                        let ip_str = ip.to_string();
                        // Filter out container and virtual bridge network adapters to select the physical uplink
                        if !name.starts_with("docker")
                            && !name.starts_with("veth")
                            && !name.starts_with("virbr")
                            && !name.starts_with("br-")
                        {
                            primary_ip = Some(ip_str);
                            break;
                        } else if fallback_ip.is_none() {
                            fallback_ip = Some(ip_str);
                        }
                    }
                }
            }
            curr = ifa.ifa_next;
        }

        libc::freeifaddrs(ifaddrs);
        primary_ip.or(fallback_ip)
    }
}

pub struct LocalIpCollector;

impl Collector for LocalIpCollector {
    fn id(&self) -> ModuleId {
        ModuleId::LocalIp
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let ip = detect_local_ip()?;
        Some(ModuleOutput {
            id: ModuleId::LocalIp,
            label: "Local IP".to_string(),
            value: ip,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_local_ip_live() {
        // Function executes without panics
        let _ = detect_local_ip();
    }
}
