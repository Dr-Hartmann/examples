pub fn find_word(s: &str, index: usize) -> &str {
    let bytes = s.as_bytes();
    let mut cur = index + 1;
    let mut start = 0;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            cur -= 1;
            if cur <= 1 {
                return &s[start..i];
            }
            start = i + 1;
        }
    }

    &s[start..]
}
