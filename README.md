# FerrisFetch

FerrisFetch is a fast, lightweight system information fetch tool written in Rust for Linux systems. It queries system metrics directly from virtual filesystems (`/proc`, `/sys`) and standard POSIX interfaces (`libc`) without spawning shell subprocesses.

---

## Latest Changes (v0.2.0)

- **Structured JSON Output**: Added `--json` CLI flag across all collectors without external dependencies.
- **Accurate GPU Resolution**: Native `pci.ids` database parser resolving vendor/device hex IDs, preventing ACPI motherboard labels (`Onboard - Video`) from overriding graphics processor model names.
- **WSL2 & Host Detection**: Automatic hypervisor detection for WSL2 and ARM device-tree boards.
- **New Package Managers**: Added package counting for **Homebrew** and **Gentoo** (`emerge`).
- **Foreground Palette Blocks**: Solid foreground block rendering (`███`) in `ColorsCollector` for universal dark/light theme visibility.
- **New Distro Logos**: Added ASCII art for **NixOS**, **Kali Linux**, **FreeBSD**, **Slackware**, **Artix Linux**, and **Zorin OS**.

*For complete version history, see [CHANGELOG.md](CHANGELOG.md).*

---

## Features

- **Direct kernel probing**: Reads `/proc`, `/sys`, and POSIX `libc` calls directly without spawning shell subprocesses.
- **Fast package counts**: Reads local package database files directly (`dpkg/status`, `pacman/local`, `apk`, `flatpak`, `snap`) without network calls or database locks.
- **Dynamic layout engine**: Computes column alignment and ANSI visible widths dynamically with automatic vertical fallback on narrow terminals (< 60 columns).
- **Distro logos & Ferris mascot**: Includes compact ASCII art logos for major Linux distributions (Arch, Debian, Ubuntu, Fedora, Mint, RHEL, Rocky, Alma, EndeavourOS, Manjaro, openSUSE, Alpine, Gentoo, Void, Pop!_OS) and the Ferris mascot.
- **Resilient fallback design**: Modules degrade gracefully when optional hardware, environment variables, or metadata files are missing.

---

## Supported Distribution Families

FerrisFetch is built and tested across the following Linux distribution families:

- **Debian Family**: Debian, Ubuntu, Linux Mint, Pop!_OS
- **Red Hat Family**: Fedora, RHEL, Rocky Linux, AlmaLinux, CentOS Stream
- **Arch Family**: Arch Linux, EndeavourOS, Manjaro
- **Android**: Termux (ARM64 & x86_64)
- **Independent Distributions**: Alpine Linux, Void Linux, Gentoo, openSUSE

---

## Installation

### Fedora / RHEL / CentOS (Copr)

Enable the official Copr repository and install:

```bash
sudo dnf copr enable kk376/ferrisfetch && sudo dnf install -y ferrisfetch
```

---

### Pre-Built Packages & Binaries

Direct packages and release binaries are available under [`releases/`](releases/) and the [GitHub Releases](https://github.com/kk376/ferrisfetch/releases) page:

- **Debian / Ubuntu / Linux Mint / Pop!_OS** (`.deb`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.2.0/ferrisfetch_0.2.0-1_amd64.deb
  sudo dpkg -i ferrisfetch_0.2.0-1_amd64.deb
  ```
  *(Or install local file: `sudo dpkg -i releases/ferrisfetch_0.2.0-1_amd64.deb`)*

- **Arch Linux / Manjaro / EndeavourOS** (`.pkg.tar.zst`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.2.0/ferrisfetch-0.2.0-1-x86_64.pkg.tar.zst
  sudo pacman -U ferrisfetch-0.2.0-1-x86_64.pkg.tar.zst
  ```
  *(Or install local file: `sudo pacman -U releases/ferrisfetch-0.2.0-1-x86_64.pkg.tar.zst`)*

- **Android (Termux ARM64)** (`.deb`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.2.0/ferrisfetch_0.2.0-1_termux_aarch64.deb
  dpkg -i ferrisfetch_0.2.0-1_termux_aarch64.deb
  ```
  *(Or install direct binary: `curl -fsSL https://github.com/kk376/ferrisfetch/releases/download/v0.2.0/ferrisfetch-termux-arm64 -o $PREFIX/bin/ferrisfetch && chmod +x $PREFIX/bin/ferrisfetch`)*

- **Standalone Static Binary** (Any 64-bit Linux / musl):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.2.0/ferrisfetch-linux-musl-x86_64
  chmod +x ferrisfetch-linux-musl-x86_64
  sudo mv ferrisfetch-linux-musl-x86_64 /usr/local/bin/ferrisfetch
  ```

---

### Build from Source

Requires Rust 1.75.0+ and `gcc`.

```bash
git clone https://github.com/kk376/ferrisfetch.git
cd ferrisfetch
cargo build --release
sudo cp target/release/ferrisfetch /usr/local/bin/
```

---

## Usage

Run FerrisFetch directly:

```bash
cargo run --release
# or after copying target/release/ferrisfetch to your PATH:
ferrisfetch
```

### CLI Options

| Flag / Option | Description |
| :--- | :--- |
| `-m, --modules <LIST>` | Select and order specific modules (e.g. `os,kernel,cpu,memory`) |
| `-d, --disable <LIST>` | Disable specific modules from output (e.g. `gpu,disk`) |
| `-l, --logo <NAME>` | Override ASCII logo (e.g. `arch`, `debian`, `ferris`, `ubuntu`, `fedora`, `tux`, `none`) |
| `--no-logo` | Suppress the ASCII logo and print only system information |
| `--no-color` | Disable ANSI color escapes |
| `--disk-path <PATH>` | Target filesystem path for disk statistics (default: `/`) |
| `--list-modules` | Print all available information modules and exit |
| `--json` | Output system information in structured JSON format |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

### Examples

**Select specific modules in custom order:**
```bash
ferrisfetch -m os,cpu,memory,disk
```

**Output structured JSON for scripts or status bars:**
```bash
ferrisfetch --json
```

**Disable GPU and packages modules:**
```bash
ferrisfetch -d gpu,packages
```

**Override logo with the Ferris mascot:**
```bash
ferrisfetch --logo ferris
```

**Query a specific mount point for disk usage:**
```bash
ferrisfetch --disk-path /home
```

---

## Color and TTY Behavior

- **Interactive Terminals**: Colors and bold accents are enabled automatically when stdout is a TTY.
- **Redirected Output**: When stdout is redirected to a file or pipe (e.g. `ferrisfetch > output.txt`), ANSI escape codes are automatically stripped.
- **Force Color**: Setting `CLICOLOR_FORCE=1` or `FORCE_COLOR=1` preserves ANSI escapes even when piped.
- **Explicit Disable**: Passing `--no-color` or setting `NO_COLOR=1` or `TERM=dumb` disables all ANSI color escapes.

---

## Information Modules & Detection Strategies

| Module | Detection Strategy | Fallback |
| :--- | :--- | :--- |
| **Title** | `$USER` / `$LOGNAME` / `getpwuid` and `uname(2)` / `/proc/sys/kernel/hostname` | `"user@localhost"` |
| **OS** | `/etc/os-release` and `/usr/lib/os-release` parsing | `/etc/debian_version`, `/etc/redhat-release`, `uname` |
| **Kernel** | POSIX `libc::uname` release and machine fields | None required |
| **Host** | `/sys/devices/virtual/dmi/id/product_name` and devicetree model | Board name or omitted |
| **Uptime** | Floating-point parse of `/proc/uptime` | `libc::sysinfo` uptime |
| **Packages** | Local DB scans: `/var/lib/dpkg/status`, `/var/lib/pacman/local`, RPM DB, APK DB, flatpak, snap | `dpkg-query`, `rpm -qa`, `xbps-query` |
| **Shell** | Ancestor process scan via `/proc/<pid>/status` & `comm`, `$SHELL` | Formatted shell name |
| **Terminal** | `$TERM_PROGRAM`, environment signatures (Alacritty, Kitty, Konsole, etc.), process tree | `$TERM` variable |
| **Desktop / WM** | `$XDG_CURRENT_DESKTOP`, Wayland sockets (`SWAYSOCK`, `HYPRLAND_INSTANCE_SIGNATURE`), process scan | Omitted if headless |
| **CPU** | `/proc/cpuinfo` parsing (`model name`, `Hardware`, `Processor`, core count, sockets) | Sanitized model string |
| **GPU** | PCI class scan (`0x03xxxx`) in `/sys/bus/pci/devices/` with vendor mapping | `lspci -mm` query |
| **Memory** | `/proc/meminfo` active memory calculation (`MemTotal - MemAvailable`) | Pre-3.14 buffer/cache calculation |
| **Disk** | POSIX `libc::statvfs` on target mount path | Omitted if unreadable |
| **Theme** | GTK 3/4 `settings.ini`, KDE `kdeglobals`, XFCE `xsettings.xml`, GSettings, `$GTK_THEME` | Omitted if not configured |
| **Icons** | GTK 3/4 `settings.ini`, KDE `kdeglobals`, XFCE `xsettings.xml`, GSettings icon-theme | Omitted if not configured |
| **Colors** | Terminal 8-color palette block renderer | Disabled if color is off |

## Shell Completions

FerrisFetch provides comprehensive shell completions for Bash, Zsh, and Fish with full support for flags, modules, and logo identifiers.

### Bash
```bash
# Load in current session
source completions/ferrisfetch.bash

# Or install system-wide
sudo cp completions/ferrisfetch.bash /usr/share/bash-completion/completions/ferrisfetch
```

### Zsh
```zsh
# Add to your fpath in ~/.zshrc before compinit
fpath=(/path/to/ferrisfetch/completions $fpath)
autoload -Uz compinit && compinit

# Or install system-wide
sudo cp completions/_ferrisfetch /usr/share/zsh/site-functions/_ferrisfetch
```

### Fish
```fish
# Install for current user
cp completions/ferrisfetch.fish ~/.config/fish/completions/

# Or install system-wide
sudo cp completions/ferrisfetch.fish /usr/share/fish/vendor_completions.d/
```

---

## Packaging

Distribution packaging manifests and build instructions are documented in [`packaging/README.md`](packaging/README.md):

- **Arch Linux (AUR)**: [`packaging/arch/`](packaging/arch/) (`PKGBUILD`, `.SRCINFO`)
- **Debian / Ubuntu**: [`packaging/debian/`](packaging/debian/) (`control`, `rules`, `changelog`)
- **Fedora / RHEL (Copr)**: [`packaging/rpm/`](packaging/rpm/) (`ferrisfetch.spec`)
- **Android (Termux)**: [`packaging/termux/`](packaging/termux/) (`build.sh`)
- **Nix / NixOS**: [`packaging/nix/`](packaging/nix/) (`default.nix`, `flake.nix`)
- **Void Linux**: [`packaging/void/`](packaging/void/) (`template`)
- **Alpine Linux**: [`packaging/alpine/`](packaging/alpine/) (`APKBUILD`)
- **Gentoo Linux**: [`packaging/gentoo/`](packaging/gentoo/) (`ferrisfetch-0.2.0.ebuild`)
- **Homebrew**: [`packaging/homebrew/`](packaging/homebrew/) (`ferrisfetch.rb`)

---

## Development & Verification

### Run unit and integration tests

```bash
cargo test
```

### Check code formatting

```bash
cargo fmt --check
```

### Run linter with zero warnings tolerance

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Build release binary

```bash
cargo build --release
```

---

## License

MIT
