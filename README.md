# Speedfetch

A fast, pretty system info fetcher for Linux terminals with RGB gradient ASCII logos.

## Features

- **78 authentic ASCII logos** from [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- **Per-distro gradient colors** matched to each distro family (Arch blue-cyan, Fedora blue-aqua, Ubuntu orange-red, etc.)
- **Aligned label columns** for clean, readable output
- **Panels**: System, Session, Hardware, Display
- **Output formats**: terminal display, JSON, TOML
- **Static gradient rendering** — beautiful colors, no animation overhead
- **Bare mode** (`--no-logo`) for minimal output with aligned labels
- **Export to file** via `--save`
- **Color control** (`--color auto|always|never`)

### Panel fields

| Panel | Fields |
|-------|--------|
| System | OS, Host, Kernel, Arch, Init, Packages (pacman, dpkg, rpm, apk, nix, flatpak, snap) |
| Session | User@Host, Shell, Terminal, DE/WM, Uptime, Locale |
| Hardware | CPU, GPU, Memory, Disk |
| Display | Resolution, Font |

## Usage

```bash
# Run from source
cargo run

# Run binary
./target/release/speedfetch

# Specify a distro logo/theme
cargo run -- --distro arch
cargo run -- -d ubuntu

# List all available distro presets
cargo run -- --list

# Output as JSON
cargo run -- --type json

# Output as TOML
cargo run -- --type toml

# Save output to a file
cargo run -- --save output.txt
cargo run -- --type json --save system.json

# Minimal output (no logo, no borders)
cargo run -- --no-logo
cargo run -- --bare
cargo run -- -b

# Disable colors (e.g. for piping)
cargo run -- --color never

# Force colors even when piping
cargo run -- --color always

# Build release
cargo build --release
```

### CLI options

```
Usage: speedfetch [OPTIONS]

Options:
  -d, --distro <DISTRO>  Distro to display (overrides auto-detection)
      --list             List available distro presets
      --type <FORMAT>    Output format (json, toml)
      --save <FILE>      Save output to file
  -b, --no-logo          Hide logo, show info only (bare output)
      --color <COLOR>    When to colorize output [auto, always, never] [default: auto]
  -h, --help             Print help
  -V, --version          Print version
```

## Info sources

| Field | Source |
|-------|--------|
| OS | `/etc/os-release` (ID, VERSION_ID, VERSION_CODENAME) |
| Host | `hostname --fqdn` |
| Kernel | `uname -r` |
| Arch | `uname -m` |
| Init | `/proc/1/comm` |
| Packages | `pacman -Q`, `dpkg --list`, `rpm -qa`, `apk info`, `nix-store`, `flatpak list`, `snap list` (smart ordering by distro) |
| Shell | `$SHELL` or `/proc/$$/cmdline` |
| Terminal | `$TERM_PROGRAM`, `$TERMINAL`, `$DESKTOP_SESSION` fallback |
| DE/WM | `$XDG_CURRENT_DESKTOP`, Hyprland/Sway/i3 detection, `$DESKTOP_SESSION`, `$WAYLAND_DISPLAY` |
| CPU | `/proc/cpuinfo` (x86/ARM/PowerPC) + `lscpu` fallback |
| GPU | `lspci` or sysfs PCI class scan with vendor lookup |
| Memory | `/proc/meminfo` (MemTotal / MemAvailable) |
| Disk | `statvfs` on `/` |
| Uptime | `/proc/uptime` |
| Resolution | `xrandr`, `wlr-randr` |
| Font | `gsettings` font-name query |
| Locale | `$LANG` |

## Install

```bash
cargo build --release
sudo cp target/release/speedfetch /usr/local/bin/
```

## License

See [LICENSE](LICENSE)
