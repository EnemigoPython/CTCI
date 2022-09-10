// prompt: Given an image represented by an N x N matrix, where each pixel in the 
// image is represented by an integer, write a method to rotate the image by 90
// degrees. Can you do this in place?

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
