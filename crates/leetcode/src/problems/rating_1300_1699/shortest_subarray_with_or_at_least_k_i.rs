// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-i

pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        const BITS: usize = 32;

        let n = nums.len();
        let mut bit_count = [0_i32; BITS];
        let mut left = 0_usize;
        let mut result = i32::MAX;
        let mut curr_or = 0_i32;

        for right in 0..n {
            let x = nums[right] as u32;
            for (b, count) in bit_count.iter_mut().enumerate() {
                if ((x >> b) & 1) == 1 {
                    *count += 1;
                    curr_or |= 1_i32 << b;
                }
            }

            while left <= right && curr_or >= k {
                result = result.min((right - left + 1) as i32);

                let y = nums[left] as u32;
                for (b, count) in bit_count.iter_mut().enumerate() {
                    if ((y >> b) & 1) == 1 {
                        *count -= 1;
                    }
                }

                curr_or = 0;
                for (b, &count) in bit_count.iter().enumerate() {
                    if count > 0 {
                        curr_or |= 1_i32 << b;
                    }
                }

                left += 1;
            }
        }

        if result == i32::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
