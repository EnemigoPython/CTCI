// prompt: Given an image represented by an N x N matrix, where each pixel in the 
// image is represented by an integer, write a method to rotate the image by 90
// degrees. Can you do this in place?

pub fn rotate_matrix<const N: usize>(m: [[i32; N]; N]) -> [[i32; N]; N] {
    // [
    //     [0, 1],
    //     [0, 1]
    // ]
    for i in m {
        for j in i {
            println!("{j}");
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_test() {
        let matrix_one = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ];
        let res_one = rotate_matrix(matrix_one);
        assert_eq!(res_one, [
            [13, 9, 5, 1],
            [14, 10, 6, 2],
            [15, 11, 7, 3],
            [16, 12, 8, 4]
        ]);

        let matrix_two = [
            [0, 1],
            [0, 1]
        ];
        let res_two = rotate_matrix(matrix_two);
        assert_eq!(res_two, [
            [0, 0],
            [1, 1]
        ]);

        let matrix_three = [
            [4, 4, 4, 3, 4],
            [3, 3, 3, 4, 3],
            [1, 2, 3, 4, 5],
            [1, 1, 1, 2, 1],
            [0, 0, 0, 1, 0]
        ];
        let res_three = rotate_matrix(matrix_three);
        assert_eq!(res_three, [
            [0, 1, 1, 3, 4],
            [0, 1, 2, 3, 4],
            [0, 1, 3, 3, 4],
            [1, 2, 4, 4, 3],
            [0, 1, 5, 3, 4]
        ]);
    }
}
