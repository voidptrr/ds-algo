// https://leetcode.com/problems/concatenation-of-array

pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len() * 2];

        let mut outer_index = 0;
        for item in result.iter_mut().take(nums.len() * 2) {
            if outer_index == nums.len() {
                outer_index = 0;
            }

            *item = nums[outer_index];
            outer_index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
