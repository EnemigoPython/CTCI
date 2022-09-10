// prompt: Implement a method to perform basic string compression using the
// counts of repeated characters. For example, the string `aabcccccaaa` would
// become `a2b1c5a3`. If the "compressed" string would not become smaller than
// the original string, your method should return the original string. You can
// assume the string has only uppercase and lowercase letters (a-z).

pub fn compress_string(s: String) -> String {
    let mut compressed = String::with_capacity(s.len());
    let mut curr_letter = s.as_bytes()[0] as char;
    let mut count = 0;
    for c in s.chars() {
        if c == curr_letter {
            count += 1;
        } else {
            compressed.push_str(&format!("{}{}", curr_letter, count));
            curr_letter = c;
            count = 1;
        }
        if compressed.len() >= s.len() { return s }
    }
    if compressed.len() + 2 >= s.len() {
        s
    } else {
        compressed.push_str(&format!("{}{}", curr_letter, count));
        compressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASES: [(&str, &str); 10] = [
        ("aabcccccaaa", "a2b1c5a3"),
        ("abcde", "abcde"),
        ("yyyeewxxxddassss", "y3e2w1x3d2a1s4"),
        ("xxgeaf", "xxgeaf"),
        ("iiiixwwwwe", "i4x1w4e1"),
        ("aabAAAbbCCCC", "a2b1A3b2C4"),
        ("ggeeaa", "ggeeaa"),
        ("jjjaabb", "j3a2b2"),
        ("llldee", "llldee"),
        ("bbbkkpooo", "b3k2p1o3")
    ];

    #[test]
    fn test_compress() {
        for (input, expected_res) in TEST_CASES {
            let res = compress_string(String::from(input));
            assert_eq!(res, expected_res);
        }
    }
}
