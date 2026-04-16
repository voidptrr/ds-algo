// https://leetcode.com/problems/count-special-quadruplets

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut diff_map = HashMap::<i32, i32>::new();
        let mut ans = 0;

        for b in (1..=nums.len() - 3).rev() {
            let c = b + 1;
            for d in c + 1..nums.len() {
                let diff = nums[d] - nums[c];
                *diff_map.entry(diff).or_insert(0) += 1;
            }

            for a in 0..b {
                let sum = nums[a] + nums[b];
                ans += diff_map.get(&sum).copied().unwrap_or(0);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
    }
}
