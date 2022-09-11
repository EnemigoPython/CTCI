// prompt: Assume you have a method `isSubstring` which checks if one word is a 
// substring of another. Given two strings, s1 and s2, write code to check if s2
// is a rotation of s1 using only one call to `isSubstring` (e.g. "waterbottle"
// is a rotation of "erbottlewat").

pub fn string_idx(s: &str, i: usize) -> char {
    s.as_bytes()[i] as char
}

pub fn is_substring(s1: &str, s2: &str) -> bool {
    s1.contains(&s2)
}

pub fn is_rotated_string(s1: String, s2: String) -> bool {
    println!("{} {}", s1, s2);
    if s1.len() != s2.len() || s1 == s2 { return false }
    let first_char = string_idx(&s1, 0);
    let mut inclusive_idx = 0;
    for c in s2.chars() {
        let compare_c = string_idx(&s1, inclusive_idx);
        if c == compare_c {
            inclusive_idx += 1;
        } else {
            inclusive_idx = if c == first_char { 1 } else { 0 };
        }
    }
    if inclusive_idx == 0 { return false }
    let str_range = 0..s2.len()-inclusive_idx;
    is_substring(&s1, &s2[str_range])
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASES: [(&str, &str, bool); 12] = [
        ("waterbottle", "erbottlewat", true),
        ("erbottlewat", "waterbottle", true),
        ("erbotwattle", "waterbottle", false),
        ("steven", "evenst", true),
        ("ststeven", "evenst", false),
        ("crazy", "azycr", true),
        ("rosewater", "waterrose", true),
        ("roseswater", "rosewater", false),
        ("iphone", "phonei", true),
        ("phoine", "iphone", false),
        ("dreams", "msdrea", true),
        ("loopy", "opylo", true)
    ];

    #[test]
    fn test_rotation() {
        for (s1, s2, expected_res) in TEST_CASES {
            let res = is_rotated_string(String::from(s1), String::from(s2));
            assert_eq!(res, expected_res);
        }
    }
}
