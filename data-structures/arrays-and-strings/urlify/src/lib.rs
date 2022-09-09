// prompt: Write a method to replace all spaces in a string with '%20'. You may assume that
// the string has sufficient space at the end to hold the additional characters, and that
// you are given the "true" length of the string.
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
