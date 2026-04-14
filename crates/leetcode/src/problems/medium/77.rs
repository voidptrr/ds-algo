// https://leetcode.com/problems/combinations

pub struct Solution;

impl Solution {
    fn backtrack(
        n: i32,
        k: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start: i32,
    ) {
        if path.len() == (k as usize) {
            result.push(path.clone());
            return;
        }

        for number in start..=n {
            path.push(number);
            Self::backtrack(n, k, result, path, number + 1);
            path.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut path = Vec::<i32>::new();

        Self::backtrack(n, k, &mut result, &mut path, 1);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utils::sorted_nested_i32;

    #[test]
    fn example_one() {
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];

        assert_eq!(
            sorted_nested_i32(Solution::combine(4, 2)),
            sorted_nested_i32(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec![vec![1]];
        assert_eq!(Solution::combine(1, 1), expected);
    }
}
