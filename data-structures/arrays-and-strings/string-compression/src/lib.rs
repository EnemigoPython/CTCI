// prompt: Implement a method to perform basic string compression using the
// counts of repeated characters. For example, the string `aabcccccaaa` would
// become `a2b1c5a3`. If the "compressed" string would not become smaller than
// the original string, your method should return the original string. You can
// assume the string has only uppercase and lowercase letters (a-z).

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
