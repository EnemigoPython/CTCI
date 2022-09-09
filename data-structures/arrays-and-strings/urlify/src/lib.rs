// prompt: Write a method to replace all spaces in a string with '%20'. You may assume that
// the string has sufficient space at the end to hold the additional characters, and that
// you are given the "true" length of the string.
// EXAMPLE:
// Input:   "Mr John Smith      ", 13
// Output:  "Mr%20John%20Smith"

pub fn urlify(input: String, len: usize) -> String {
    let mut new = String::with_capacity(input.capacity());
    let mut i = 0;
    for c in input.chars() {
        i += 1;
        if i > len { return new }
        if c == ' ' {
            new.push_str("%20");
        } else {
            new.push(c);
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASES: [(&str, &str, usize); 5] = [
        ("Mr John Smith      ", "Mr%20John%20Smith", 13),
        ("Mr John Smith      ", "Mr%20John%20Smith%20", 14),
        ("This is Sparta", "This%20is%20Sparta", 14),
        ("This is Sparta", "This%20is%20Spar", 12),
        ("Double  Space", "Double%20%20Space", 13)
    ];

    #[test]
    fn urlify_test() {
        for (_in, expected_out, len) in TEST_CASES {
            let res = urlify(String::from(_in), len);
            assert_eq!(res.as_str(), expected_out);
        }
    }
}
