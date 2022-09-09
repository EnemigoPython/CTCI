// prompt: Given a string, write a function to check if it is a permutation of a palindrome. A palindrome
// is a word or phrase that is the same forwards and backwards. A permutation is a rearrangement of letters.
// The palindrome does not need to be limited to just dictionary words. You can ignore casing and non-letter
// characters.
// EXAMPLE:
// Input:   Tact Coa
// Output:  True (permutations: "taco cat", "atco cta", etc.)
use std::collections::HashMap;

pub fn is_palindrome_permutation(s: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASES: [(&str, bool); 10] = [
        ("Tact Coa", true),
        ("Satire: Veritas", true),
        ("expensive", false),
        ("AXA has sha", true),
        ("UwU", true),
        ("A man a plan a canal: Panama", true),
        ("Definitely not symmetrical", false),
        ("each meach", true),
        ("Ska man", false),
        ("fred is dead", false)
    ];

    #[test]
    fn check_palindrome_permutation() {
        for (str, val) in TEST_CASES {
            let res = is_palindrome_permutation(String::from(str));
            assert_eq!(res, val);
        }
    }
}
