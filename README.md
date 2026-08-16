# FerrisFetch

FerrisFetch is a fast, lightweight system information fetch tool written in Rust for Linux systems. It queries system metrics directly from virtual filesystems (`/proc`, `/sys`) and standard POSIX interfaces (`libc`) without spawning shell subprocesses.

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

## Installation & Building

### Pre-built Packages & Executables

Pre-built binaries and native distribution packages are available in the [`releases/`](releases/) directory:

#### Debian / Ubuntu / Linux Mint / Pop!_OS
```bash
sudo dpkg -i releases/ferrisfetch_0.1.0-1_amd64.deb
```

#### Fedora / RHEL / Rocky Linux / AlmaLinux
```bash
sudo dnf install ./releases/ferrisfetch-0.1.0-1.x86_64.rpm
# or with rpm:
sudo rpm -i releases/ferrisfetch-0.1.0-1.x86_64.rpm
```

#### Arch Linux / Manjaro / EndeavourOS
```bash
sudo pacman -U releases/ferrisfetch-0.1.0-1-x86_64.pkg.tar.zst
```

#### Android / Termux (ARM64)
```bash
# 1. Update Termux packages and install curl
pkg update && pkg upgrade -y
pkg install -y curl

# 2. Download the package from GitHub Releases
curl -LO https://github.com/Kk376/ferrisfetch/releases/download/v0.1.0/ferrisfetch_0.1.0-1_termux_aarch64.deb

# 3. Install it with Termux's dpkg
dpkg -i ferrisfetch_0.1.0-1_termux_aarch64.deb

# 4. Run FerrisFetch
ferrisfetch
```

Or run the one-liner directly:
```bash
pkg update && pkg upgrade -y && pkg install -y curl && curl -fsSL https://github.com/Kk376/ferrisfetch/releases/download/v0.1.0/ferrisfetch-termux-arm64 -o $PREFIX/bin/ferrisfetch && chmod +x $PREFIX/bin/ferrisfetch && ferrisfetch
```

#### Standalone Binary (Any x86_64 Linux Distribution)
```bash
chmod +x releases/ferrisfetch-linux-musl-x86_64
sudo cp releases/ferrisfetch-linux-musl-x86_64 /usr/local/bin/ferrisfetch
```

### Prerequisites for Building

- Rust 1.75.0 or later (uses standard library `std::io::IsTerminal`)
- Standard Linux C library headers (`libc`)

### Build from source

```bash
git clone https://github.com/kk376/ferrisfetch.git
cd ferrisfetch
cargo build --release
```

The compiled release binary is located at `target/release/ferrisfetch`.

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
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

### Examples

**Select specific modules in custom order:**
```bash
ferrisfetch -m os,cpu,memory,disk
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

Distribution packaging templates and recipes are available in the `packaging/` directory:

- **Debian / Ubuntu**: [`packaging/debian/`](packaging/debian/) (Debian source package with `control`, `rules`, `changelog`, `compat`, `copyright`)
- **Fedora / RHEL / CentOS**: [`packaging/rpm/ferrisfetch.spec`](packaging/rpm/ferrisfetch.spec) (RPM spec file with cargo release build and completion manifests)
- **Arch Linux / AUR**: [`packaging/arch/PKGBUILD`](packaging/arch/PKGBUILD) and [`packaging/arch/.SRCINFO`](packaging/arch/.SRCINFO)

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

## Limitations

- **Headless Servers**: Graphical desktop and window manager fields are omitted on headless systems or TTY sessions without display servers.
- **GPU in Sandboxes**: Containers or virtual machines lacking access to `/sys/bus/pci` or `lspci` will report virtual display devices or omit GPU information.
- **Container Packages**: In minimal containers without package databases or status files, the package count module cleanly produces no output rather than failing.

---

## License

MIT
