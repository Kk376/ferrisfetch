# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-08-19

### Added
- **OS Architecture**: Displays system architecture alongside distribution name (e.g. `OS: Ubuntu 24.04.4 LTS x86_64`).
- **Shell Version**: Resolves active shell version (e.g. `Shell: zsh 5.9`, `Shell: bash 5.2.21`).
- **Display Resolution & Refresh Rate**: Probes connected displays and refresh rates (e.g. `Display: 1920x1080 @ 60Hz`).
- **Window Manager Module (`WM`)**: Resolves active window managers including Mutter, KWin, Xfwm4, Sway, Hyprland, and `WSLg (Weston)`.
- **GPU VRAM & Clock Frequency**: Displays GPU memory capacity and max graphics clock (e.g. `GPU0: AMD Radeon 660M (512 MiB) @ 1.900GHz`, `GPU1: NVIDIA GeForce RTX 2050 (4 GiB) @ 2.100GHz`).
- **Swap Memory Module**: Displays total and active used swap partition/file memory.
- **Partition Disk Enumeration**: Discovers all active physical and virtual partitions labeled sequentially (`Disk0`, `Disk1`, `Disk2`), formatting WSL Windows drives directly (e.g. `(C)`, `(D)`).
- **Physical Battery Detection**: Probes battery percentage and status from sysfs while automatically filtering out Microsoft Hyper-V virtual batteries in WSL.
- **Local IP Module**: Probes primary local IPv4 address via standard POSIX interface enumeration without subprocesses.

## [0.3.0] - 2026-08-19

### Added
- **Multi-Socket CPU Scaling**: Formats multi-socket CPU systems as `<n>x <CPU Name> (<Total Threads>)` (e.g. `3x AMD EPYC 9654 (384)`).
- **CPU Clock Speed**: Added frequency resolution (`@ nGHz`) from `/proc/cpuinfo` and `cpufreq` sysfs.
- **Dynamic Sequential GPU Indexing**: Assigns sequential indices (`GPU0`, `GPU1`, `GPU2`, ...) without skipping numbers.
- **iGPU `GPU0` Priority**: Integrated graphics always occupy `GPU0` and scale across multi-socket systems (`GPU0: <n>x <iGPU Name>`).
- **dGPU Automatic Grouping**: Automatically groups identical discrete GPUs into a single line (`GPU<index>: <n>x <dGPU Name>`).
- **Sub-30ms WSL GPU Caching**: Persistent caching for discrete GPU queries in WSL2, reducing execution time from ~1.7s to under 30ms.

### Changed
- Stripped redundant integrated graphics marketing strings (`with Radeon Graphics`, `with Intel UHD Graphics`) from CPU model lines for a cleaner terminal silhouette.

## [0.2.5] - 2026-08-19

### Added
- Native WSL2 hybrid GPU detection (`GpuCollector`) resolving both integrated graphics (e.g. `AMD Radeon 660M`/`680M`/`780M`, `Intel Iris Xe`/`UHD`) and discrete NVIDIA graphics (e.g. `NVIDIA GeForce RTX 2050`/`3050`/`4050`) via the native Windows driver bridge without extra Linux drivers.

## [0.2.0] - 2026-08-19

### Added
- `--json` CLI flag for structured JSON output across all enabled modules without external dependencies.
- Native `pci.ids` database parser resolving PCI vendor/device hex pairs to human-readable graphics cards without spawning subprocesses.
- WSL2 hypervisor and host motherboard model identification in `HostCollector`.
- Package manager counting support for **Homebrew** (`Cellar`) and **Gentoo** (`/var/db/pkg`).
- Foreground solid color block glyphs (`███`) in `ColorsCollector` for consistent light/dark terminal rendering.
- Built-in ASCII logos for **NixOS**, **Kali Linux**, **FreeBSD**, **Slackware**, **Artix Linux**, and **Zorin OS**.

### Fixed
- Fixed GPU detection prioritizing motherboard ACPI DMI slot labels (e.g. `Onboard - Video` on ASUS/Dell laptops) over actual graphics processor model names.

## [0.1.0] - 2026-08-16

### Added

- Core system information fetch engine implemented in Rust without spawning shell subprocesses.
- System metrics collectors:
  - **Title**: Username and hostname resolution via environment variables and `getpwuid`/`uname`.
  - **OS**: Linux distribution identification via `/etc/os-release` and `/usr/lib/os-release`.
  - **Host**: Hardware model and chassis parsing from `/sys/devices/virtual/dmi/id/` and device-tree.
  - **Kernel**: Release and architecture parsing from POSIX `libc::uname`.
  - **Uptime**: Accurate uptime calculation from `/proc/uptime` and `libc::sysinfo`.
  - **Packages**: Direct file-based package counting for Debian (`dpkg/status`), Arch (`pacman/local`), Red Hat (RPM database), Alpine (`apk`), Flatpak, and Snap.
  - **Shell**: Current shell process detection and version extraction via `/proc/<pid>/status` and `$SHELL`.
  - **Terminal**: Active terminal emulator detection via environment variables (`TERM_PROGRAM`, Alacritty, Kitty, Konsole, Foot) and process tree walking.
  - **Desktop / WM**: Desktop environment and window manager detection via `$XDG_CURRENT_DESKTOP`, Wayland socket signatures, and process scans.
  - **CPU**: Multi-socket, core count, and model name parsing from `/proc/cpuinfo` with vendor string sanitization.
  - **GPU**: Direct PCI sysfs scan (`/sys/bus/pci/devices`) with vendor ID mapping and fallback detection.
  - **Memory**: Accurate memory consumption calculation (`MemTotal - MemAvailable`) from `/proc/meminfo`.
  - **Disk**: Mount point capacity and utilization querying using POSIX `libc::statvfs`.
  - **Colors**: 8-color terminal palette block rendering.
- Layout and rendering engine:
  - Dynamic side-by-side logo and metric alignment.
  - ANSI escape code stripping for accurate visible character width calculation.
  - Automatic vertical layout fallback for narrow terminal displays (< 60 columns).
  - Terminal color auto-detection respecting `NO_COLOR`, `CLICOLOR_FORCE`, and non-TTY stdout redirection.
- Built-in ASCII logos:
  - Ferris the Rust mascot.
  - Distribution art for Arch, Debian, Ubuntu, Linux Mint, Fedora, RHEL, Rocky Linux, AlmaLinux, EndeavourOS, Manjaro, openSUSE, Alpine, Gentoo, Void Linux, Pop!_OS, and generic Tux.
- Command-line interface (`clap` derive):
  - `-m, --modules`: Module selection and ordering.
  - `-d, --disable`: Selective module disabling.
  - `-l, --logo`: ASCII logo override by distribution name or alias.
  - `--no-logo`: Logo suppression.
  - `--no-color`: ANSI color disabling.
  - `--disk-path`: Target path selection for disk metrics.
  - `--list-modules`: Available module enumeration.
- Comprehensive test suite:
  - Unit tests covering parsing logic across edge cases and malformed files.
  - Integration tests with synthetic procfs and sysfs fixtures for 15+ Linux distributions.
  - CLI flag combination tests using `assert_cmd`.
- Packaging and CI infrastructure:
  - GitHub Actions CI workflow for formatting, clippy, unit/integration testing, and release builds.
  - Release workflow building standalone GNU and Musl binaries, Debian (`.deb`), Red Hat (`.rpm`), Arch Linux (`.pkg.tar.zst`), and Android / Termux (`.deb` & ARM64 binary) packages with SHA256 checksums.
  - Arch Linux `PKGBUILD` and Debian packaging specifications.
