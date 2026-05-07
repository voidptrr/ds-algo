// https://leetcode.com/problems/k-radius-subarray-averages

pub struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut out = vec![-1; nums.len()];
        let k = k as usize;

        let window = 2 * k + 1;
        if window > nums.len() {
            return out;
        }

        let mut sum: i64 = 0;
        for item in nums.iter().take(window) {
            sum += *item as i64;
        }
        out[k] = (sum / window as i64) as i32;

        let mut left = 1usize;
        let mut right = window;

        while right < nums.len() {
            sum -= nums[left - 1] as i64;
            sum += nums[right] as i64;

            let center = left + k;
            out[center] = (sum / window as i64) as i32;

            left += 1;
            right += 1;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::get_averages(vec![100000], 0), vec![100000]);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::get_averages(vec![8], 100000), vec![-1]);
    }
}
