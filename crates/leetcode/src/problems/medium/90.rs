// https://leetcode.com/problems/subsets-ii

pub struct Solution;

impl Solution {
    fn backtrack(
        input: &[i32],
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start: usize,
    ) {
        result.push(path.clone());

        for index in start..input.len() {
            if index > start && input[index] == input[index - 1] {
                continue;
            }

            path.push(input[index]);
            Self::backtrack(input, path, result, index + 1);
            path.pop();
        }
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        let mut nums_mut = nums.clone();
        nums_mut.sort();

        Self::backtrack(&nums_mut, &mut path, &mut result, 0);

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
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];

        assert_eq!(
            sorted_nested_i32(Solution::subsets_with_dup(vec![1, 2, 2])),
            sorted_nested_i32(expected)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sorted_nested_i32(Solution::subsets_with_dup(vec![0])),
            vec![vec![], vec![0]]
        );
    }
}
