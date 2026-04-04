// https://leetcode.com/problems/combination-sum-iii

pub struct Solution;

impl Solution {
    fn backtrack(
        k: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start: i32,
        remain: i32,
    ) {
        // Only numbers 1 through 9 are used.
        // Each number is used at most once.

        if remain < 0 {
            return;
        }

        if remain == 0 && path.len() == (k as usize) {
            result.push(path.clone());
            return;
        }

        for cand in start..=9 {
            path.push(cand);
            Self::backtrack(k, result, path, cand + 1, remain - cand);
            path.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::<i32>::new();

        Self::backtrack(k, &mut result, &mut path, 1, n);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted_nested(mut input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        input.sort();
        input
    }

    #[test]
    fn example_one() {
        let expected = vec![vec![1, 2, 4]];
        assert_eq!(Solution::combination_sum3(3, 7), expected);
    }

    #[test]
    fn example_two() {
        let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        assert_eq!(
            sorted_nested(Solution::combination_sum3(3, 9)),
            sorted_nested(expected)
        );
    }

    #[test]
    fn example_three() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum3(4, 1), expected);
    }
}
