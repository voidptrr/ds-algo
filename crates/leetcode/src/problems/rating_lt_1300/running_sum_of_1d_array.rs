// https://leetcode.com/problems/running-sum-of-1d-array

pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(nums.len());

        for index in 0..nums.len() {
            if index == 0 {
                result.push(nums[index]);
            } else {
                result.push(result[index - 1] + nums[index]);
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
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }
}
