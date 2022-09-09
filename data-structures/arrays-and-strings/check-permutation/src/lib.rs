// prompt: Given two strings, write a method to decide if one is a permutation of the other.
use std::collections::HashMap;

pub fn check_permutation(a: String, b: String) -> bool {
    if a.len() != b.len() || a == b { return false }
    let mut a_hash: HashMap<char, usize> = HashMap::new();
    let mut b_hash: HashMap<char, usize> = HashMap::new();
    for c in a.chars() {
        if let Some(x) = a_hash.get_mut(&c) {
            *x += 1;
        } else {
            a_hash.insert(c, 1);
        }
    }
    for c in b.chars() {
        if let Some(x) = a_hash.get_mut(&c) {
            if let Some(y) = b_hash.get_mut(&c) {
                if y >= x { return false }
                *y += 1;
            } else {
                b_hash.insert(c, 1);
            }
        } else {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static POSITIVE_CASES: [(&str, &str); 5] = [
        ("Hi", "iH"), 
        ("abc", "bca"), 
        ("Reality show", "Real shitowy"), 
        ("Very true", "true Very"), 
        ("tame", "mate")
    ];

    static NEGATIVE_CASES: [(&str, &str); 5] = [
        ("Hii", "iH i"), 
        ("Not the same", "Nota thesame "), 
        ("What", "Wat"), 
        ("Case matters here", "case matters Here"), 
        ("Very close to true", "eVry clost to true")
    ];

    #[test]
    fn is_permutation() {
        for (p1, p2) in POSITIVE_CASES {
            let result = check_permutation(String::from(p1), String::from(p2));
            assert!(result);
        }
    }

    #[test]
    fn not_permutation() {
        for (p1, p2) in NEGATIVE_CASES {
            let result = check_permutation(String::from(p1), String::from(p2));
            assert!(!result);
        }
    }
}
