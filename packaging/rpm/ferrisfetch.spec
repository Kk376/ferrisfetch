%global debug_package %{nil}

Name:           ferrisfetch
Version:        0.4.1
Release:        1%{?dist}
Summary:        Fast, lightweight Linux system information fetch tool written in Rust

License:        MIT
URL:            https://github.com/kk376/ferrisfetch
Source0:        https://github.com/kk376/ferrisfetch/archive/refs/tags/v%{version}.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires:  cargo >= 1.75.0
BuildRequires:  rust >= 1.75.0
BuildRequires:  gcc

%description
FerrisFetch is a fast, zero-runtime-dependency CLI system information fetch tool
written in Rust, specifically designed for Linux distributions. It gathers system
metrics including OS release, kernel version, CPU, GPU, memory, disk usage,
package managers, desktop environment, uptime, and shell information, formatting
them cleanly alongside colorful ANSI distribution ASCII logos.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release

%check
cargo test --release

%install
# Install executable binary
install -Dpm 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install shell completions
install -Dpm 0644 completions/%{name}.bash %{buildroot}%{_datadir}/bash-completion/completions/%{name}
install -Dpm 0644 completions/_%{name} %{buildroot}%{_datadir}/zsh/site-functions/_%{name}
install -Dpm 0644 completions/%{name}.fish %{buildroot}%{_datadir}/fish/vendor_completions.d/%{name}.fish

# Install documentation
install -Dpm 0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/bash-completion/completions/%{name}
%dir %{_datadir}/zsh/site-functions
%{_datadir}/zsh/site-functions/_%{name}
%dir %{_datadir}/fish/vendor_completions.d
%{_datadir}/fish/vendor_completions.d/%{name}.fish

%changelog
* Sun Aug 16 2026 FerrisFetch Packaging Team <packaging@ferrisfetch.rs> - 0.1.0-1
- Initial RPM release for version 0.1.0
- Added modular system metric collectors
- Added multi-distro ANSI 256-color ASCII art
- Added Bash, Zsh, and Fish shell completions
