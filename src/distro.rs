pub fn distro() -> String {
    std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .find_map(|line| {
            line.strip_prefix("ID=")
                .map(|v| v.trim_matches(['"', '\'']).to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn ansi_color() -> Option<(u8, u8, u8)> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    let line = content.lines().find(|l| l.starts_with("ANSI_COLOR="))?;
    let val = line.strip_prefix("ANSI_COLOR=")?.trim_matches(['"', '\'']);

    let parts: Vec<u8> = val.split(';').filter_map(|s| s.parse().ok()).collect();
    match parts.as_slice() {
        [38, 2, r, g, b] => Some((*r, *g, *b)),
        _ => None,
    }
}
