pub fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix_chars: Vec<char> = strs[0].chars().collect();

    for s in strs.iter().skip(1) {
        if prefix_chars.len() > s.len() {
            prefix_chars.truncate(s.len());
        }

        let mismatch_index = prefix_chars
            .iter()
            .zip(s.chars())
            .position(|(p, c)| *p != c);

        if let Some(i) = mismatch_index {
            prefix_chars.truncate(i);
        }

        if prefix_chars.is_empty() {
            return String::new();
        }
    }

    prefix_chars.into_iter().collect()
}
