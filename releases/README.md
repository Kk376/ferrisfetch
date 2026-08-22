# FerrisFetch Releases (v0.8.6)

Official pre-built release binaries, native OS distribution packages, source archives, and build recipes for **FerrisFetch v0.8.6** — a fast, lightweight Linux system information fetch tool written in Rust.

---

## 📦 Artifact Index

### 1. Source Code Archives

| Archive | Format | Description |
| :--- | :--- | :--- |
| `ferrisfetch-0.8.6-source.tar.gz` | `.tar.gz` | Clean source code distribution tarball (v0.8.6) |
| `ferrisfetch-0.8.6-source.zip` | `.zip` | Clean source code distribution zip archive (v0.8.6) |

### 2. Standalone Direct Executables

| Binary | Target / Architecture | Linking | Description |
| :--- | :--- | :--- | :--- |
| `ferrisfetch-termux-arm64` | Android / Termux (`aarch64`) | Static (musl) | 100% statically linked standalone binary for ARM64 Android phones |
| `ferrisfetch-android-aarch64` | Android / Termux (`aarch64`) | Symlink | Symlink to `ferrisfetch-termux-arm64` |
| `ferrisfetch-linux-musl-aarch64` | Universal Linux (`aarch64`) | Symlink | Symlink to `ferrisfetch-termux-arm64` |
| `ferrisfetch-linux-musl-x86_64` | Universal Linux (`x86_64`) | Static (musl) | 100% statically linked standalone binary (zero libc/glibc dependencies) |
| `ferrisfetch-debian-x86_64` | Debian / Ubuntu (`x86_64`) | Dynamic (glibc) | Dynamic ELF executable targeting Debian/Ubuntu family systems |
| `ferrisfetch-debian` | Debian / Ubuntu (`x86_64`) | Symlink | Symlink to `ferrisfetch-debian-x86_64` |
| `ferrisfetch-fedora-x86_64` | Fedora / RHEL (`x86_64`) | Dynamic (glibc) | Dynamic ELF executable targeting Fedora/Red Hat family systems |
| `ferrisfetch-fedora` | Fedora / RHEL (`x86_64`) | Symlink | Symlink to `ferrisfetch-fedora-x86_64` |
| `ferrisfetch-redhat` | Red Hat / RHEL (`x86_64`) | Symlink | Symlink to `ferrisfetch-fedora-x86_64` |
| `ferrisfetch-rhel` | Red Hat / RHEL (`x86_64`) | Symlink | Symlink to `ferrisfetch-fedora-x86_64` |
| `ferrisfetch-arch-x86_64` | Arch Linux (`x86_64`) | Dynamic (glibc) | Stripped dynamic ELF executable targeting Arch Linux |
| `ferrisfetch-arch` | Arch Linux (`x86_64`) | Symlink | Symlink to `ferrisfetch-arch-x86_64` |

### 3. Native Distribution Packages & Archives

| Package / Recipe | Target Distribution | Description |
| :--- | :--- | :--- |
| `ferrisfetch_0.8.6-1_amd64.deb` | Debian / Ubuntu / Mint / Pop!_OS | Native `.deb` package installable via `dpkg -i` or `apt install` |
| `ferrisfetch_0.8.6-1_termux_aarch64.deb` | Android / Termux (`aarch64`) | Native Termux `.deb` package installable via `dpkg -i` in Termux |
| `ferrisfetch-0.8.6-1.x86_64.rpm` | Fedora / RHEL / CentOS / Rocky / Alma | Native `.rpm` package installable via `rpm -i` or `dnf install` |
| `ferrisfetch-0.8.6-1-x86_64.pkg.tar.zst` | Arch Linux / Manjaro / EndeavourOS | Native Pacman package (Zstandard compressed) installable via `pacman -U` |
| `ferrisfetch-0.8.6-1-x86_64.pkg.tar.gz` | Arch Linux / Manjaro / EndeavourOS | Native Pacman package (Gzip compressed fallback) |
| `ferrisfetch-0.8.6-fedora-x86_64.tar.gz` | Fedora / RHEL (`x86_64`) | Binary release bundle containing executable, documentation, and spec |
| `PKGBUILD` | Arch Linux AUR / Source | Standard Arch Linux build recipe for `makepkg` |
| `ferrisfetch.spec` | Fedora / RPM Build | Standard RPM specification recipe for `rpmbuild` |

---

## 🔒 Verification & Integrity

Verify the integrity of all downloaded release artifacts using `SHA256SUMS.txt`:

```bash
sha256sum -c SHA256SUMS.txt
```

---

## 🚀 Quick Installation

### Debian / Ubuntu / Linux Mint / Pop!_OS
```bash
sudo dpkg -i ferrisfetch_0.8.6-1_amd64.deb
ferrisfetch
```

### Android / Termux (ARM64)
```bash
# Option A: Install via Termux package
pkg update && pkg upgrade -y
pkg install -y curl
curl -LO https://github.com/kk376/ferrisfetch/releases/download/v0.8.6/ferrisfetch_0.8.6-1_termux_aarch64.deb
dpkg -i ferrisfetch_0.8.6-1_termux_aarch64.deb
ferrisfetch

# Option B: One-liner to download standalone ARM64 executable
pkg update && pkg upgrade -y && pkg install -y curl && curl -fsSL https://github.com/kk376/ferrisfetch/releases/download/v0.8.6/ferrisfetch-termux-arm64 -o $PREFIX/bin/ferrisfetch && chmod +x $PREFIX/bin/ferrisfetch && ferrisfetch
```

### Fedora / RHEL / Rocky Linux / AlmaLinux
```bash
# Using RPM directly:
sudo rpm -Uvh ferrisfetch-0.8.6-1.x86_64.rpm

# Or using DNF:
sudo dnf install ./ferrisfetch-0.8.6-1.x86_64.rpm
ferrisfetch
```

### Arch Linux / Manjaro / EndeavourOS
```bash
sudo pacman -U ferrisfetch-0.8.6-1-x86_64.pkg.tar.zst
ferrisfetch
```

### Universal Standalone Binary (Any Linux x86_64)
```bash
chmod +x ferrisfetch-linux-musl-x86_64
./ferrisfetch-linux-musl-x86_64
# Or install to PATH:
sudo install -m 755 ferrisfetch-linux-musl-x86_64 /usr/local/bin/ferrisfetch
```

### Building from Source Archive
```bash
tar -xzf ferrisfetch-0.1.0-source.tar.gz
cd ferrisfetch-0.1.0
cargo build --release
./target/release/ferrisfetch
```
