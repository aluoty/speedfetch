use std::fs;

pub fn logo() -> Vec<String> {
    let distro_info = fs::read_to_string("/etc/os-release").unwrap();
    let mut distro = "unknown";

    for line in distro_info.lines() {
        if line.starts_with("ID=") {
            distro = line.trim_start_matches("ID=").trim_matches('"');
            break;
        }
    }

    if distro.contains("h") {
        return vec![
            "  _____        _                  ".to_string(),
            " |  ___|__  __| | ___  _ __ __ _  ".to_string(),
            " | |_ / _ \\/ _` |/ _ \\| '__/ _` | ".to_string(),
            " |  _|  __/ (_| | (_) | | | (_| | ".to_string(),
            " |_|  \\___|\\__,_|\\___/|_|  \\__,_| ".to_string(),
            "                                  ".to_string(),
        ];
    }


        vec![
            "  _   _                  ".to_string(),
            " | \\ | | ___  _ __   ___ ".to_string(),
            " |  \\| |/ _ \\| '_ \\ / _ \\".to_string(),
            " | |\\  | (_) | | | |  __/".to_string(),
            " |_| \\_|\\___/|_| |_|\\___|".to_string(),
            "                         ".to_string(),
        ]
}