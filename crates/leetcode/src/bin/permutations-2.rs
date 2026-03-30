// https://leetcode.com/problems/permutations-ii

struct Solution;

impl Solution {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &Vec<i32>,
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

        let mut nums_mut = nums.clone();
        nums_mut.sort();

        Self::backtrack(&mut result, &nums_mut, &mut cand, &mut used);

        result
    }
}

fn main() {
    println!("{:?}", Solution::permute_unique(vec![1, 1, 2]));
    println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
}
