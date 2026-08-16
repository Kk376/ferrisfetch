# FerrisFetch

FerrisFetch is a fast, lightweight system information fetch tool written in Rust for Linux systems. It reads system metrics directly from virtual filesystems (`/proc`, `/sys`) and standard POSIX APIs (`libc`) without spawning shell subprocesses or requiring runtime dependencies.

```text
    _~^~^~_       kk376@MSI-Thin-A15
\) /  o o  \ (/   ------------------
  '_   -   _'     OS: Ubuntu 24.04.4 LTS
  / '-----' \     Kernel: 6.18.33.2-microsoft-standard-WSL2
                  Uptime: 45 mins
                  Packages: 1018 (dpkg)
                  Shell: zsh
                  Terminal: Windows Terminal
                  CPU: AMD Ryzen 5 7535HS with Radeon Graphics (12)
                  GPU: Microsoft Corporation Basic Render Driver
                  Memory: 1.73 GiB / 7.36 GiB (24%)
                  Disk: 16.3 GiB / 1006.9 GiB (2%)
```

## Features

- **Direct kernel probing**: Uses `/proc`, `/sys`, and POSIX `libc` calls for instantaneous metrics with zero shell invocations.
- **Fast package counts**: Reads local package database files directly (`dpkg/status`, `pacman/local`, `apk`, `flatpak`, `snap`) without network calls or database locks.
- **Zero-overhead layout rendering**: Computes column alignment and ANSI visible widths dynamically with automatic vertical fallback on narrow terminals (< 60 columns).
- **Distro logos & Ferris mascot**: Includes compact ASCII art logos for major Linux distributions (Arch, Debian, Ubuntu, Fedora, Mint, RHEL, Rocky, Alma, EndeavourOS, Manjaro, openSUSE, Alpine, Gentoo, Void, Pop!_OS) and the Ferris mascot.

## Installation

### Prerequisites
- Rust 1.75.0 or later

### Building from source

```bash
git clone https://github.com/kk376/ferrisfetch.git
cd ferrisfetch
cargo build --release
```

The compiled binary will be located at `target/release/ferrisfetch`.

## Usage

Run FerrisFetch directly:

```bash
ferrisfetch
```

### Command-line Options

| Flag | Description |
| :--- | :--- |
| `-m, --modules <LIST>` | Select and order specific modules (e.g. `os,kernel,cpu,memory`) |
| `-d, --disable <LIST>` | Disable specific modules (e.g. `gpu,disk`) |
| `-l, --logo <NAME>` | Override ASCII logo (e.g. `arch`, `debian`, `ferris`, `ubuntu`, `tux`, `none`) |
| `--no-logo` | Suppress the ASCII logo and print only system information |
| `--no-color` | Disable ANSI color escapes (also honors the `NO_COLOR` environment variable) |
| `--disk-path <PATH>` | Target filesystem path for disk statistics (default: `/`) |
| `--list-modules` | Print all available information modules and exit |

### Examples

Display only OS, CPU, and RAM metrics:
```bash
ferrisfetch -m os,cpu,memory
```

Override logo with the Ferris mascot:
```bash
ferrisfetch --logo ferris
```

Disable GPU and Disk probing:
```bash
ferrisfetch -d gpu,disk
```

Output plain text for scripts or piping:
```bash
ferrisfetch --no-color --no-logo
```

## Architecture

See [docs/architecture.md](docs/architecture.md) for full implementation details, module probing mechanisms, and design principles.

## License

MIT
