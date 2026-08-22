# FerrisFetch

FerrisFetch is a fast, lightweight system information fetch tool written in Rust for Linux, macOS, and native Windows systems. It queries system metrics directly from virtual filesystems (`/proc`, `/sys`), Win32 APIs, and standard interfaces without spawning shell subprocesses.

---

## Latest Changes (v0.7.0)

- **Localized Installation Timestamps**: Formats system installation time (`Installed:`) according to the user's local timezone and daylight saving time via native POSIX (`localtime_r` / `tm_gmtoff`) and Win32 (`GetTimeZoneInformation`) APIs, replacing raw UTC+0 display (suggested by [@Laynsb](https://github.com/Laynsb)).
- **Native Windows NT Support**: Direct Win32 hardware and subsystem detection without spawning shell subprocesses.
- **Windows Package Managers**: Native package counting for **WinGet**, **Chocolatey**, and **Cargo**.
- **Windows ASCII Art Logos**: High-resolution ANSI ASCII art for **Windows 11**, **Windows 10**, and **Classic Windows**.

*For complete version history, see [CHANGELOG.md](CHANGELOG.md).*

---

## Features

- **Direct OS kernel probing**: Reads `/proc`, `/sys`, POSIX APIs, and Win32 registry/system calls directly without spawning shell subprocesses.
- **Fast package counts**: Reads local package database files directly (`dpkg/status`, `pacman/local`, `apk`, `flatpak`, `snap`, `winget`, `chocolatey`, `cargo`, `npm`, `pip`) without network calls or locks.
- **Dynamic layout engine**: Computes column alignment and ANSI visible widths dynamically with automatic vertical fallback on narrow terminals (< 60 columns).
- **Distro & OS logos**: Includes compact ASCII art logos for Windows (11, 10, Classic), major Linux distributions (Arch, Debian, Ubuntu, Fedora, Mint, RHEL, Rocky, Alma, EndeavourOS, Manjaro, openSUSE, Alpine, Gentoo, Void, Pop!_OS), and the Ferris mascot.
- **Resilient fallback design**: Modules degrade gracefully when optional hardware, environment variables, or metadata files are missing.

---

## Supported Operating Systems

FerrisFetch is built and tested across:

- **Windows**: Windows 11, Windows 10, Windows Server (native Win32 x86_64)
- **Debian Family**: Debian, Ubuntu, Linux Mint, Pop!_OS
- **Red Hat Family**: Fedora, RHEL, Rocky Linux, AlmaLinux, CentOS Stream
- **Arch Family**: Arch Linux, EndeavourOS, Manjaro
- **Android**: Termux (ARM64 & x86_64)
- **Independent Distributions**: Alpine Linux, Void Linux, Gentoo, openSUSE

---

## Installation

### Windows (Manual / WinGet)

*WinGet package submission is currently under review by Microsoft and will soon be live (`winget install ferrisfetch`).*

To install and run manually in PowerShell, run these 3 short commands one by one:

#### Step 1: Download
```powershell
curl.exe -LO https://github.com/kk376/ferrisfetch/releases/download/v0.7.0/ferrisfetch-windows-x86_64.zip
```

#### Step 2: Extract
```powershell
tar.exe -xf ferrisfetch-windows-x86_64.zip
```

#### Step 3: Run
```powershell
.\ferrisfetch.exe
```

### Ubuntu / Linux Mint / Pop!_OS (PPA)

Enable the official Launchpad PPA and install:

```bash
sudo add-apt-repository -y ppa:kushagra376/ferrisfetch && sudo apt update && sudo apt install -y ferrisfetch
```

---

### Fedora (Copr)

Enable the official Copr repository and install:

```bash
sudo dnf copr enable kk376/ferrisfetch && sudo dnf install -y ferrisfetch
```

---

### Pre-Built Packages & Binaries

Direct packages and release binaries are available under [`releases/`](releases/) and the [GitHub Releases](https://github.com/kk376/ferrisfetch/releases) page:

- **Debian / Ubuntu / Linux Mint / Pop!_OS** (`.deb`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.5.0/ferrisfetch_0.5.0-1_amd64.deb
  sudo dpkg -i ferrisfetch_0.5.0-1_amd64.deb
  ```
  *(Or install local file: `sudo dpkg -i releases/ferrisfetch_0.5.0-1_amd64.deb`)*

- **Arch Linux / Manjaro / EndeavourOS** (`.pkg.tar.zst`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.5.0/ferrisfetch-0.5.0-1-x86_64.pkg.tar.zst
  sudo pacman -U ferrisfetch-0.5.0-1-x86_64.pkg.tar.zst
  ```
  *(Or install local file: `sudo pacman -U releases/ferrisfetch-0.5.0-1-x86_64.pkg.tar.zst`)*

- **Android (Termux ARM64)** (`.deb`):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.5.0/ferrisfetch_0.5.0-1_termux_aarch64.deb
  dpkg -i ferrisfetch_0.5.0-1_termux_aarch64.deb
  ```
  *(Or install direct binary: `curl -fsSL https://github.com/kk376/ferrisfetch/releases/download/v0.5.0/ferrisfetch-termux-arm64 -o $PREFIX/bin/ferrisfetch && chmod +x $PREFIX/bin/ferrisfetch`)*

- **Standalone Static Binary** (Any 64-bit Linux / musl):
  ```bash
  curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.5.0/ferrisfetch-linux-musl-x86_64
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
| **OS** | `/etc/os-release` and `/usr/lib/os-release` parsing with architecture | `/etc/debian_version`, `/etc/redhat-release`, `uname` |
| **Host** | `/sys/devices/virtual/dmi/id/product_name` and devicetree model | Board name or omitted |
| **Kernel** | POSIX `libc::uname` release and machine fields | None required |
| **Installed** | Root filesystem creation timestamp via `statx(2)` (`stx_btime`) and installer logs | Distribution log birth time |
| **Uptime** | Floating-point parse of `/proc/uptime` | `libc::sysinfo` uptime |
| **Packages** | Local DB scans: `/var/lib/dpkg/status`, `/var/lib/pacman/local`, RPM DB, APK DB, flatpak, snap | `dpkg-query`, `rpm -qa`, `xbps-query` |
| **Shell** | Ancestor process scan via `/proc/<pid>/status` & `comm`, `$SHELL` | Formatted shell name & version |
| **Display** | DRM sysfs modes, `xrandr`, and `wlr-randr` refresh rate probing | Omitted if headless |
| **Desktop** | `$XDG_CURRENT_DESKTOP`, metadata version files, and session type | Omitted if headless |
| **WM** | Active window manager detection (Mutter, KWin, Sway, Hyprland, WSLg) | Process scan |
| **Terminal** | Dedicated environment signatures, `/proc` process ancestry, `$TERM` | `$TERM` variable |
| **CPU** | `/proc/cpuinfo` parsing (model, clean brand, sockets, core count, clock freq) | Sanitized model string |
| **GPU** | Sysfs PCI scan (`0x03xxxx`), local `pci.ids` lookup, VRAM and clock speeds | `lspci -mm` query |
| **Memory** | `/proc/meminfo` active memory calculation (`MemTotal - MemAvailable`) | Pre-3.14 buffer/cache calculation |
| **Swap** | `/proc/meminfo` swap statistics (`SwapTotal - SwapFree`) | Omitted if swap is 0 |
| **Disk** | Sequential physical and virtual partition discovery via `statvfs` | Target mount path |
| **Battery** | Direct `/sys/class/power_supply` capacity and charging status | Omitted if no battery |
| **Local IP** | POSIX `getifaddrs` active interface address enumeration | Omitted if offline |
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
- **Gentoo Linux**: [`packaging/gentoo/`](packaging/gentoo/) (`ferrisfetch-0.2.5.ebuild`)
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

## Community Acknowledgements & Recommendations

Special thanks to community contributors for architectural recommendations and feature suggestions:

- **[@Laynsb](https://github.com/Laynsb)**:
  - **System Installation Date Module (`Installed`)**: Recommended adding the OS installation date module, probing root filesystem creation timestamp (`stx_btime`) and distribution installer records with human-readable relative time deltas (e.g. `23 Jan 2026, 12:22 AM (211 days ago)`).
  - **Localized Installation Timestamps**: Recommended native local timezone and daylight saving time conversion for the `Installed` module, ensuring timestamps reflect the user's localized wall-clock time instead of raw UTC+0.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture guidelines, code standards, and PR instructions.

## License

MIT. See [LICENSE](LICENSE).
