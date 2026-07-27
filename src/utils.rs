pub fn strip_ansi(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == 0x1b {
            i += 1;
            while i < bytes.len() {
                let b = bytes[i];
                i += 1;
                if (b >= 0x41 && b <= 0x5A) || (b >= 0x61 && b <= 0x7A) {
                    break;
                }
            }
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }

    unsafe { String::from_utf8_unchecked(result) }
}
