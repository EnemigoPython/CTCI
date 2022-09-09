// prompt: There are three types of edits that can be performed on strings: insert a character, remove
// a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false
use std::cmp;

pub fn string_idx(str: &String, idx: usize) -> char {
    str.as_bytes()[idx] as char
}

pub fn one_away(before: String, after: String) -> bool {
    let len_diff = before.len() as i32 - after.len() as i32;
    if len_diff > 1 || len_diff < -1 { return false }
    let mut diffs = 0;
    let mut adjust: i32 = 0;
    let min_len = cmp::min(before.len(), after.len());
    for i in 0..min_len {
        let before_c = string_idx(&before, i);
        let after_c = string_idx(&after, (i as i32 + adjust) as usize);
        if before_c != after_c {
            if diffs > 0 { return false }
            if len_diff > 0 {
                if string_idx(&before, i+1) != after_c { return false }
                adjust -= 1;
            }
            if len_diff < 0 {
                if before_c != string_idx(&after, i+1) { return false }
                adjust += 1;
            }
            diffs += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASES: [(&str, &str, bool); 12] = [
        ("pale", "ple", true),
        ("pales", "pale", true),
        ("pale", "bale", true),
        ("pale", "bake", false),
        ("space", "pace", true),
        ("pace", "space", true),
        ("dream", "cream", true),
        ("dream", "creams", false),
        ("real", "really", false),
        ("really", "real", false),
        ("trial", "triad", true),
        ("gun", "fund", false)
    ];

    #[test]
    fn test_one_away() {
        for (s1, s2, expected_res) in TEST_CASES {
            let res = one_away(String::from(s1), String::from(s2));
            assert_eq!(expected_res, res);
        }
    }
}
