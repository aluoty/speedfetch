pub fn strip_ansi(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(&next) = chars.peek() {
                chars.next();
                if next.is_ascii_lowercase() || next.is_ascii_uppercase() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
