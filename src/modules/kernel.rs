use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};
use std::ffi::CStr;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnameInfo {
    pub sysname: String,
    pub hostname: String,
    pub kernel_release: String,
    pub architecture: String,
}

/// Retrieves POSIX utsname system metadata via direct libc uname syscall.
/// Avoids spawning subprocesses (`uname -r`) and parsing `/proc/version` directly.
pub fn get_uname_info() -> Option<UnameInfo> {
    unsafe {
        let mut uts = MaybeUninit::<libc::utsname>::uninit();
        if libc::uname(uts.as_mut_ptr()) != 0 {
            return None;
        }
        let uts = uts.assume_init();

        let sysname = CStr::from_ptr(uts.sysname.as_ptr())
            .to_string_lossy()
            .into_owned();
        let hostname = CStr::from_ptr(uts.nodename.as_ptr())
            .to_string_lossy()
            .into_owned();
        let kernel_release = CStr::from_ptr(uts.release.as_ptr())
            .to_string_lossy()
            .into_owned();
        let architecture = CStr::from_ptr(uts.machine.as_ptr())
            .to_string_lossy()
            .into_owned();

        Some(UnameInfo {
            sysname,
            hostname,
            kernel_release,
            architecture,
        })
    }
}

pub struct KernelCollector;

impl Collector for KernelCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Kernel
    }

    fn collect(&self, _ctx: &FetchContext) -> Option<ModuleOutput> {
        let uname = get_uname_info()?;
        Some(ModuleOutput {
            id: ModuleId::Kernel,
            label: "Kernel".to_string(),
            value: uname.kernel_release,
            custom_rendered: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uname_info_live() {
        let uname = get_uname_info();
        assert!(uname.is_some());
        let info = uname.unwrap();
        assert!(!info.kernel_release.is_empty());
        assert!(!info.architecture.is_empty());
    }
}
