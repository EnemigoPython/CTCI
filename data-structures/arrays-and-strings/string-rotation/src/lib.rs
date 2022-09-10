// prompt: Assume you have a method `isSubstring` which checks if one word is a 
// substring of another. Given two strings, s1 and s2, write code to check if s2
// is a rotation of s1 using only one call to `isSubstring` (e.g. "waterbottle"
// is a rotation of "erbottlewat").

pub fn is_rotated_string(s1: String, s2: String) -> bool {
    true
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
