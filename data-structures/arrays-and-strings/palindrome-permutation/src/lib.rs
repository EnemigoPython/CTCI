// prompt: Given a string, write a function to check if it is a permutation of a palindrome. A palindrome
// is a word or phrase that is the same forwards and backwards. A permutation is a rearrangement of letters.
// The palindrome does not need to be limited to just dictionary words. You can ignore casing and non-letter
// characters.
// EXAMPLE:
// Input:   Tact Coa
// Output:  True (permutations: "taco cat", "atco cta", etc.)
use std::collections::HashMap;

pub fn is_palindrome_permutation(s: String) -> bool {
    let mut s_hash: HashMap<char, usize> = HashMap::new();
    let mut odd_pairs: usize = 0;
    for c in s.chars() {
        if !c.is_ascii_alphabetic() { continue }
        let mut lower_c = c.clone();
        lower_c.make_ascii_lowercase();
        if let Some(x) = s_hash.get_mut(&lower_c) {
            *x += 1;
            if *x % 2 == 0 {
                odd_pairs -= 1;
            } else {
                odd_pairs += 1;
            }
        } else {
            s_hash.insert(lower_c, 1);
            odd_pairs += 1;
        }
    }
    odd_pairs < 2
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
            println!("{}", str);
            assert_eq!(res, val);
        }
    }
}
