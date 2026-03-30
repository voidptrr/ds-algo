// https://leetcode.com/problems/permutations-ii

pub struct Solution;

impl Solution {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &[i32],
        cand: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if cand.len() == input.len() {
            result.push(cand.clone());
            return;
        }

        for (index, &n) in input.iter().enumerate() {
            if used[index] {
                continue;
            }

            if index > 0 && input[index] == input[index - 1] && !used[index - 1]
            {
                continue;
            }

            used[index] = true;
            cand.push(n);
            Self::backtrack(result, input, cand, used);
            cand.pop();
            used[index] = false;
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut cand: Vec<i32> = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];

        let mut nums_sorted = nums;
        nums_sorted.sort();

        Self::backtrack(&mut result, &nums_sorted, &mut cand, &mut used);

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
        let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        assert_eq!(
            sorted_nested(Solution::permute_unique(vec![1, 1, 2])),
            sorted_nested(expected)
        );
    }

    #[test]
    fn unique_input_still_permuted() {
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        assert_eq!(
            sorted_nested(Solution::permute_unique(vec![1, 2, 3])),
            sorted_nested(expected)
        );
    }
}
