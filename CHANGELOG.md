# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
