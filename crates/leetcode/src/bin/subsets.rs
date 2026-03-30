// https://leetcode.com/problems/subsets

struct Solution;

impl Solution {
    fn backtrack(
        input: &[i32],
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start: usize,
    ) {
        result.push(path.clone());

        for index in start..input.len() {
            path.push(input[index]);
            Self::backtrack(input, path, result, index + 1);
            path.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        Self::backtrack(&nums, &mut path, &mut result, 0);

        result
    }
}

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
}
