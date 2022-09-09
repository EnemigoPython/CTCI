// prompt: There are three types of edits that can be performed on strings: insert a character, remove
// a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false

pub fn one_away(before: String, after: String) -> bool {
    if ((before.len() as i32) - (after.len() as i32)).abs() > 2 {
        return false
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
