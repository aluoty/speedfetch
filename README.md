# Speedfetch

A fast, pretty system info fetcher for Linux terminals with RGB gradient ASCII logos.

## Features

- **78 authentic ASCII logos** from [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- **Per-distro gradient colors** matched to each distro family (Arch blue-cyan, Fedora blue-aqua, Ubuntu orange-red, etc.)
- **Aligned label columns** for clean, readable output
- **Panels**: System, Session, Hardware, Display
- **Output formats**: terminal display, JSON, TOML
- **Static gradient rendering** — beautiful colors, no animation overhead

### Panel fields

| Panel | Fields |
|-------|--------|
| System | OS, Host, Kernel, Arch, Init, Packages |
| Session | User@Host, Shell, Terminal, DE/WM, Uptime, Locale |
| Hardware | CPU, GPU, Memory, Disk |
| Display | Resolution, Font |

## Usage

```bash
# Run from source
cargo run

# Run binary
./target/debug/speedfetch
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
  -h, --help             Print help
  -V, --version          Print version
```

## Install

```bash
cargo build --release
sudo cp target/release/speedfetch /usr/local/bin/
```

## License

MIT
