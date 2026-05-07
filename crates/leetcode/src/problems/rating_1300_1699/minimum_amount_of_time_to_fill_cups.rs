// https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups

pub struct Solution;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let sum: i32 = amount.iter().sum();
        let max_val: i32 = *amount.iter().max().unwrap_or(&0);

        let half_ceil = (sum + 1) / 2;
        max_val.max(half_ceil)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }
}
