# KISS Linux Packaging for FerrisFetch

This directory contains the KISS Linux package recipe files for `ferrisfetch`.

## Package Structure

- `build`: Posix shell script executing `cargo build --release --locked` and installing binaries, shell completions, documentation, and license.
- `version`: Package version and release (`0.8.5 1`).
- `sources`: Source tarball location (`https://github.com/kk376/ferrisfetch/archive/refs/tags/v0.8.5.tar.gz`).
- `checksums`: SHA256 checksum for source tarball validation.
- `depends`: Build/runtime dependencies (`rust make`).

## Building & Testing Locally

1. Add this directory or your custom repository to `$KISS_PATH`:
   ```sh
   export KISS_PATH="/path/to/ferrisfetch/packaging/kiss:$KISS_PATH"
   ```

2. Build and install:
   ```sh
   kiss build ferrisfetch
   kiss install ferrisfetch
   ```

3. Generate or verify checksums:
   ```sh
   kiss checksum ferrisfetch
   ```

## Upstream Community Repository

This package is maintained in the KISS Community Repository:
- [kiss-community/community](https://github.com/kiss-community/community)
