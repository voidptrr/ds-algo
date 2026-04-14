// https://leetcode.com/problems/permutations

pub struct Solution;

impl Solution {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &[i32],
        cand: &mut Vec<i32>,
        used: &mut [bool],
    ) {
        if cand.len() == input.len() {
            result.push(cand.clone());
            return;
        }

        for (index, &n) in input.iter().enumerate() {
            if used[index] {
                continue;
            }

            used[index] = true;
            cand.push(n);
            Self::backtrack(result, input, cand, used);
            cand.pop();
            used[index] = false;
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut cand = Vec::<i32>::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];

        Self::backtrack(&mut result, &nums, &mut cand, &mut used);

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
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        assert_eq!(
            sorted_nested_i32(Solution::permute(vec![1, 2, 3])),
            sorted_nested_i32(expected)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sorted_nested_i32(Solution::permute(vec![0, 1])),
            vec![vec![0, 1], vec![1, 0]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
