// prompt: Write an algorithm such that if an element in an M x N matrix is 0, its entire row
// and column are set to 0.
use std::collections::HashSet;

pub fn zero_matrix<const M: usize, const N: usize>(mut m: [[i32; M]; N]) -> [[i32; M]; N] {
    let mut zero_rows: HashSet<usize> = HashSet::new();
    let mut zero_cols: HashSet<usize> = HashSet::new();
    for (i, row) in m.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == 0 {
                zero_rows.insert(i);
                zero_cols.insert(j);
            }
        }
    }
    for row in zero_rows {
        m[row] = [0; M];
    }
    for col in zero_cols {
        for i in 0..N {
            m[i][col] = 0;
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix() {
        let matrix_one = [
            [3, 4, 5],
            [2, 1, 1],
            [5, 0, 2],
            [1, 1, 1]
        ];
        let res_one = zero_matrix(matrix_one);
        assert_eq!(res_one, [
            [3, 0, 5],
            [2, 0, 1],
            [0, 0, 0],
            [1, 0, 1]
        ]);

        let matrix_two = [
            [0, 1, 2],
            [3, 2, 0],
            [4, 7, 1]
        ];
        let res_two = zero_matrix(matrix_two);
        assert_eq!(res_two, [
            [0, 0, 0],
            [0, 0, 0],
            [0, 7, 0]
        ]);

        let matrix_three = [
            [5, 4, 3, 2, 1],
            [3, 2, 1, 2, 3]
        ];
        let res_three = zero_matrix(matrix_three);
        assert_eq!(res_three, matrix_three);

        let matrix_four = [
            [2, 1],
            [0, 4],
            [5, 1],
            [2, 3]
        ];
        let res_four = zero_matrix(matrix_four);
        assert_eq!(res_four, [
            [0, 1],
            [0, 0],
            [0, 1],
            [0, 3]
        ]);

        let matrix_five = [
            [3, 5, 6, 3, 2, 0],
            [2, 1, 4, 5, 3, 2],
            [1, 0, 2, 3, 6, 0],
            [2, 0, 5, 3, 2, 5],
            [4, 4, 1, 2, 0, 1],
            [5, 8, 6, 4, 3, 2]
        ];
        let res_five = zero_matrix(matrix_five);
        assert_eq!(res_five, [
            [0, 0, 0, 0, 0, 0],
            [2, 0, 4, 5, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [5, 0, 6, 4, 0, 0]
        ])
    }
}
