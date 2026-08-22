# Void Linux Packaging for FerrisFetch

This directory contains the Void Linux `xbps-src` package template for `ferrisfetch`.

## Template Location

To build and test within a local `void-packages` tree:

```bash
git clone --depth=1 https://github.com/void-linux/void-packages.git
cd void-packages
./xbps-src binary-bootstrap

# Copy the template into srcpkgs
mkdir -p srcpkgs/ferrisfetch
cp /path/to/ferrisfetch/packaging/void/template srcpkgs/ferrisfetch/

# Build and lint
./xbps-src pkg ferrisfetch
xlint srcpkgs/ferrisfetch/template
```

## Submitting to `void-linux/void-packages`

1. Fork and clone `void-linux/void-packages`.
2. Create a branch: `git checkout -b ferrisfetch`.
3. Add `srcpkgs/ferrisfetch/template`.
4. Test the build: `./xbps-src pkg ferrisfetch`.
5. Run linter: `xlint srcpkgs/ferrisfetch/template`.
6. Commit with the standard Void message format:
   ```bash
   git commit -m "New package: ferrisfetch-0.8.5"
   ```
7. Push and open a PR against `void-linux/void-packages:master`.
