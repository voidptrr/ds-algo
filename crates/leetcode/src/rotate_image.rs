// https://leetcode.com/problems/rotate-image

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix.len();

        for row in 0..matrix.len() {
            for col in row..(size - row - 1) {
                let temp = matrix[row][col];

                matrix[row][col] = matrix[size - 1 - col][row];
                matrix[size - 1 - col][row] =
                    matrix[size - 1 - row][size - 1 - col];
                matrix[size - 1 - row][size - 1 - col] =
                    matrix[col][size - 1 - row];
                matrix[col][size - 1 - row] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut input);

        let output = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(input, output);
    }

    #[test]
    fn example_two() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut input);

        let output = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        assert_eq!(input, output);
    }
}
