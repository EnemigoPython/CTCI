// prompt: Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?
use std::collections::HashSet;

/// with hash set
pub fn is_unique_1(str: String) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for c in str.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

/// no additional data structures
pub fn is_unique_2(str: String) -> bool {
    for (i, c1) in str.chars().enumerate() {
        for (j, c2) in str.chars().enumerate() {
            if i == j { continue }
            if c1 == c2 { return false }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static POSITIVE_CASES: [&str; 5] = [
        "Hi", 
        "abc", 
        "Reality show", 
        "BMW", 
        "TtAa"
    ];

    static NEGATIVE_CASES: [&str; 5] = [
        "Hii", 
        "abcc", 
        "Unreal reality", 
        "The quick brown fox jumps over the lazy dog", 
        "Gravity shmavity"
    ];

    #[test]
    fn positive_result() {
        for test in POSITIVE_CASES {
            assert!(is_unique_1(String::from(test)));
        }
    }

    #[test]
    fn negative_result() {
        for test in NEGATIVE_CASES {
            assert!(!is_unique_1(String::from(test)));
        }
    }

    #[test]
    fn positive_result_2() {
        for test in POSITIVE_CASES {
            assert!(is_unique_1(String::from(test)));
        }
    }

    #[test]
    fn negative_result2() {
        for test in NEGATIVE_CASES {
            assert!(!is_unique_2(String::from(test)));
        }
    }
}
