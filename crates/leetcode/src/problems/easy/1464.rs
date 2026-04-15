// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums_mut = nums.clone();
        nums_mut.sort();

        let len = nums_mut.len();
        let (pre_last, last) = (nums_mut[len - 1], nums_mut[len - 2]);

        (pre_last - 1) * (last - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
