// https://leetcode.com/problems/squares-of-a-sorted-array

pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let mut left = 0;
        let mut right = nums.len() - 1;

        for r in (0..=nums.len() - 1).rev() {
            if nums[left].abs() >= nums[right].abs() {
                result[r] = nums[left].pow(2);
                left += 1;
            } else {
                result[r] = nums[right].pow(2);
                right -= 1;
            }
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
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
