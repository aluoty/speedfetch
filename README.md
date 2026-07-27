# Speedfetch

A fast, pretty system info fetcher for Linux terminals with gradient ASCII logos.

## Features

- **78 authentic ASCII logos** from [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- **Per-distro gradient colors** matched to each distro family
- **Full TOML config** with `--config` or `~/.config/speedfetch/config.toml`
- **Box layout** with Unicode borders and title bar
- **Bar/percent display** for memory, swap, battery, CPU usage
- **33 info modules** including battery, datetime, CPU usage, public IP, WiFi, BIOS
- **Logo modes**: regular, small, none
- **Key styles**: string labels, nerd-font icons, or both
- **Output formats**: terminal display, JSON, TOML
- **Export to file** via `--save`
- **Color control** (`--color auto|always|never`)

## Usage

```bash
# Run from source
cargo run

# Run release binary
./target/release/speedfetch

# Specify a distro logo
speedfetch --distro arch
speedfetch -d ubuntu

# List all available distro presets
speedfetch --list

# Show only specific fields
speedfetch --show os,kernel,cpu,memory,gpu

# Use a custom config file
speedfetch --config ~/.config/my-theme.toml

# Logo modes
speedfetch --logo small
speedfetch --logo regular
speedfetch --logo none
speedfetch --no-logo          # alias for --logo none
speedfetch -b                 # short form

# Output as JSON / TOML
speedfetch --type json
speedfetch --type toml

# Save output to file
speedfetch --save output.txt
speedfetch --type json --save system.json

# Color control
speedfetch --color never      # strip colors (for piping)
speedfetch --color always     # force colors even when piping

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
      --logo <TYPE>      Logo mode: small, regular, none
  -b, --no-logo          Hide logo, show info only (alias for --logo none)
      --color <COLOR>    When to colorize output [auto, always, never]
      --show <FIELDS>    Show only specific fields (comma-separated: os,kernel,cpu,memory,...)
      --config <FILE>    Path to custom config file
  -h, --help             Print help
  -V, --version          Print version
```

## Configuration

Speedfetch reads TOML config from `~/.config/speedfetch/config.toml` or a custom path via `--config`.

Top-level keys (`separator`, `show`) must appear **before** any `[section]` headers (TOML spec).

```toml
separator = " "
show = ["os", "host", "kernel", "uptime", "shell", "wm", "cpu", "gpu", "memory"]

[logo]
mode = "regular"           # "regular" | "small" | "none"

[key]
type = "string"            # "string" | "icon" | "both" | "none"
width = null               # force fixed key width (null = auto)
padding_left = 0

[color]
keys   = null              # override label color (null = distro theme)
values = null              # override value color  (null = distro theme)
separator = null

[layout]
boxes          = false     # wrap info in Unicode box borders
title          = false     # show user@host in box title bar
separator_line = false     # draw line between title and content

[bar]
width         = 20
char_elapsed  = "█"
char_total    = "░"
border_left   = null
border_right  = null

[percent]
type         = "number"    # "number" | "bar" | "both" | "colored"
ndigits      = 1
color_green  = "green"
color_yellow = "yellow"
color_red    = "red"
```

### Available colors

`black`, `red`, `green`, `yellow`, `blue`, `purple`, `magenta`, `cyan`, `white`, `gray`, `dim`, `bright_red`, `bright_green`, `bright_yellow`, `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`, `orange`, `pink`, `lavender`, `teal`, `lime`, `gold`, `brown`, `none`

### Example configs

See [`docs/`](docs/) for ready-made configs:

| Config | Description |
|--------|-------------|
| `default.toml` | Full reference with all options documented |
| `minimal.toml` | Essential fields only, no logo |
| `maximal.toml` | Every field, bars, colored percentages |
| `neofetch.toml` | Classic neofetch-style layout |
| `compact.toml` | Dense layout, no logo, dot-style bars |
| `colors.toml` | Vibrant color scheme with `::` separator |
| `box.toml` | Unicode box borders with title bar |

## Available fields

| Key | Field |
|-----|-------|
| `os` | Operating system |
| `host` | Hostname |
| `kernel` | Kernel version |
| `uptime` | System uptime |
| `pkgs` | Package count (pacman, dpkg, rpm, apk, nix, flatpak, snap) |
| `shell` | Current shell |
| `res` | Display resolution |
| `de` | Desktop environment |
| `wm` | Window manager |
| `wm-theme` | WM theme |
| `term` | Terminal emulator |
| `term-size` | Terminal dimensions (cols x rows) |
| `cpu` | CPU model |
| `cpu-usage` | CPU usage percentage |
| `gpu` | GPU model |
| `memory` | RAM usage |
| `swap` | Swap usage |
| `disk` | Disk usage |
| `drive` | Drive model |
| `battery` | Battery percentage |
| `font` | Font family |
| `init` | Init system (systemd, OpenRC, runit) |
| `user` | User@Host |
| `ip` | Local IP address |
| `public-ip` | Public IP address |
| `wifi` | WiFi SSID |
| `datetime` | Current date and time |
| `locale` | System locale |
| `procs` | Process count |
| `arch` | CPU architecture |
| `bios` | BIOS vendor and version |
| `board` | Motherboard vendor and name |

## Info sources

| Field | Source |
|-------|--------|
| OS | `/etc/os-release` |
| Host | `$HOSTNAME` / `hostname` |
| Kernel | `/proc/sys/kernel/osrelease` |
| Arch | `$HOSTTYPE` / `uname -m` |
| Init | `/run/systemd/system`, `/sbin/openrc`, `/proc/1/comm` |
| Packages | `pacman -Q`, `dpkg --list`, `rpm -qa`, `apk info`, `nix-store`, `flatpak list`, `snap list` |
| Shell | `$SHELL` |
| Terminal | `$TERM_PROGRAM`, Alacritty/Kitty/WezTerm/Ghostty detection |
| DE/WM | `$XDG_CURRENT_DESKTOP`, Hyprland/Sway/i3 socket detection |
| CPU | `/proc/cpuinfo`, `lscpu` |
| GPU | `lspci`, sysfs PCI class scan |
| Memory | `/proc/meminfo` |
| Disk | `df -h` on `/`, `/boot`, `/home`, `/nix`, `/var` |
| Battery | `/sys/class/power_supply` |
| Resolution | `xrandr`, `wlr-randr` |
| CPU Usage | `/proc/stat` (100ms sample) |
| Public IP | `curl ifconfig.me` |
| WiFi | `iwgetid -r`, `nmcli` |
| BIOS/Board | `/sys/class/dmi/id/` |

## Install

```bash
cargo build --release
sudo cp target/release/speedfetch /usr/local/bin/
```

## License

See [LICENSE](LICENSE)
