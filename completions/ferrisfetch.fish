# Fish shell completion script for ferrisfetch

# Disable file completions by default for ferrisfetch
complete -c ferrisfetch -f

# Modules definitions
set -l modules \
    'title\tUsername@Hostname title header' \
    'os\tOperating system and distribution name' \
    'host\tHardware product and model name' \
    'kernel\tLinux kernel release version' \
    'uptime\tSystem running time since boot' \
    'packages\tInstalled package counts' \
    'pkgs\tInstalled package counts (alias)' \
    'shell\tCurrent shell name and version' \
    'desktop\tDesktop Environment or Window Manager' \
    'de\tDesktop Environment (alias)' \
    'wm\tWindow Manager (alias)' \
    'terminal\tActive terminal emulator' \
    'term\tTerminal emulator (alias)' \
    'cpu\tProcessor model and core count' \
    'gpu\tDetected graphics hardware' \
    'memory\tSystem RAM usage statistics' \
    'mem\tSystem RAM usage (alias)' \
    'disk\tTarget filesystem storage usage' \
    'colors\tTerminal 16-color ANSI palette' \
    'palette\tTerminal color palette (alias)'

# Logos definitions
set -l logos \
    'ferris\tFerris the Crab (Rust mascot)' \
    'rust\tFerris the Crab (alias)' \
    'debian\tDebian swirl logo' \
    'ubuntu\tUbuntu circle of friends logo' \
    'linuxmint\tLinux Mint logo' \
    'mint\tLinux Mint logo (alias)' \
    'fedora\tFedora infinity logo' \
    'arch\tArch Linux logo' \
    'archlinux\tArch Linux logo (alias)' \
    'rhel\tRed Hat Enterprise Linux logo' \
    'redhat\tRed Hat Enterprise Linux (alias)' \
    'centos\tCentOS logo (alias)' \
    'rocky\tRocky Linux logo' \
    'rockylinux\tRocky Linux logo (alias)' \
    'almalinux\tAlmaLinux logo' \
    'alma\tAlmaLinux logo (alias)' \
    'endeavouros\tEndeavourOS logo' \
    'endeavour\tEndeavourOS logo (alias)' \
    'manjaro\tManjaro Linux logo' \
    'opensuse\topenSUSE chameleon logo' \
    'suse\topenSUSE logo (alias)' \
    'alpine\tAlpine Linux mountain logo' \
    'gentoo\tGentoo Linux logo' \
    'void\tVoid Linux logo' \
    'pop\tPop!_OS logo' \
    'popos\tPop!_OS logo (alias)' \
    'generic\tGeneric Linux Tux logo' \
    'tux\tTux penguin (alias)' \
    'linux\tGeneric Linux (alias)' \
    'none\tDisable ASCII logo output'

# Options and flags
complete -c ferrisfetch -s m -l modules -d 'Enable specific modules in order (comma-separated)' -r -a "$modules"
complete -c ferrisfetch -s d -l disable -d 'Disable specific modules (comma-separated)' -r -a "$modules"
complete -c ferrisfetch -s l -l logo -d 'Override the ASCII logo' -r -a "$logos"
complete -c ferrisfetch -l disk-path -d 'Target mount point or directory path for disk usage statistics' -r -F
complete -c ferrisfetch -l no-color -d 'Disable colored ANSI output'
complete -c ferrisfetch -l no-logo -d 'Do not display any ASCII logo'
complete -c ferrisfetch -l list-modules -d 'List all available information modules and exit'
complete -c ferrisfetch -s h -l help -d 'Print help'
complete -c ferrisfetch -s V -l version -d 'Print version'
