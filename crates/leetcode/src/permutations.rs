// https://leetcode.com/problems/permutations

pub struct Solution;

impl Solution {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &[i32],
        cand: &mut Vec<i32>,
    ) {
        if cand.len() == input.len() {
            result.push(cand.clone());
            return;
        }

        for &n in input {
            if !cand.contains(&n) {
                cand.push(n);
                Self::backtrack(result, input, cand);
                cand.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut cand: Vec<i32> = Vec::with_capacity(nums.len());

        Self::backtrack(&mut result, &nums, &mut cand);

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
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        assert_eq!(
            sorted_nested(Solution::permute(vec![1, 2, 3])),
            sorted_nested(expected)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sorted_nested(Solution::permute(vec![0, 1])),
            vec![vec![0, 1], vec![1, 0]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
