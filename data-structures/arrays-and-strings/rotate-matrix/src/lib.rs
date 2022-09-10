// prompt: Given an image represented by an N x N matrix, where each pixel in the 
// image is represented by an integer, write a method to rotate the image by 90
// degrees. Can you do this in place?

pub fn rotate_matrix<const N: usize>(m: [[i32; N]; N]) -> [[i32; N]; N] {
    if N < 2 { return m }
    let mut tmp: char;
    for i in 0..N/2 {
        for j in i..N-(i+1) {
            
        }
    }
    // while iter + 2 < N {
    //     for i in iter..N {

    //     }
    //     iter += 1;
    // }
    // for i in m {
    //     for j in i {
    //         println!("{j}");
    //     }
    // }
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

        let matrix_four = [
            [1, 3, 5, 7, 9, 11],
            [2, 4, 6, 8, 10, 12],
            [3, 5, 7, 9, 11, 13],
            [4, 6, 8, 10, 12, 14],
            [5, 7, 9, 11, 13, 15],
            [6, 8, 10, 12, 14, 16]
        ];
        let res_four = rotate_matrix(matrix_four);
        assert_eq!(res_four, [
            [6, 5, 4, 3, 2, 1],
            [8, 7, 6, 5, 4, 3],
            [10, 9, 8, 7, 6, 5],
            [12, 11, 10, 9, 8, 7],
            [14, 13, 12, 11, 10, 9],
            [16, 15, 14, 13, 12, 11]
        ]); // start at index 0 and do len - 1 rotations, then 1,1 and do len-3... until 0 on next row
        // 4: 3 -> 1, 5: 4 -> 2, 6: 5 -> 3 -> 1
    }
}
