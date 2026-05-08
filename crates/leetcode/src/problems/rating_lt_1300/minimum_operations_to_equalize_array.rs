// https://leetcode.com/problems/minimum-operations-to-equalize-array

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let all_equal = nums.windows(2).all(|w| w[0] == w[1]);

        if all_equal { 0 } else { 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_operations(vec![1, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::min_operations(vec![5, 5, 5]), 0);
    }
}
