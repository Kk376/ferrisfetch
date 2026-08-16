Name:           ferrisfetch
Version:        0.1.0
Release:        1%{?dist}
Summary:        Fast, lightweight Linux system information fetch tool written in Rust

License:        MIT
URL:            https://github.com/kk376/ferrisfetch
Source0:        %{name}-%{version}-fedora-x86_64.tar.gz

ExclusiveArch:  x86_64

%description
FerrisFetch is a high-performance, modular system information fetch tool
for Linux distributions including Fedora, RHEL, CentOS Stream, and Rocky Linux.
Written in pure Rust with direct sysfs, procfs, and libc integration for near-instant
execution time and minimal resource consumption.

%prep
%setup -q -c

%build
# Pre-built optimized release binary

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}
install -p -m 0755 ferrisfetch-fedora-x86_64 %{buildroot}%{_bindir}/ferrisfetch

%files
%{_bindir}/ferrisfetch

%changelog
* Sun Aug 16 2026 FerrisFetch Team <support@ferrisfetch.org> - 0.1.0-1
- Initial release of FerrisFetch for Fedora / RHEL
