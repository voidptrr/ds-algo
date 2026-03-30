// https://leetcode.com/problems/permutations

struct Solution;

impl Solution {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &Vec<i32>,
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

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
    println!("{:?}", Solution::permute(vec![0, 1]));
    println!("{:?}", Solution::permute(vec![1]));
}
