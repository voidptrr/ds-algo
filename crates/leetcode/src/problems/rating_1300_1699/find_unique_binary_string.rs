// https://leetcode.com/problems/find-unique-binary-string

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let seen: HashSet<i32> = nums
            .iter()
            .map(|s| i32::from_str_radix(s, 2).unwrap())
            .collect();

        for v in 0..(1 << nums.len()) {
            if !seen.contains(&v) {
                return format!("{:0width$b}", v, width = nums.len());
            }
        }

        unreachable!("Missing binary string not found");
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let out = Solution::find_different_binary_string(
            vec!["01", "10"].into_iter().map(str::to_owned).collect(),
        );

        assert_ne!(out, "01");
        assert_ne!(out, "10");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn example_two() {
        let out = Solution::find_different_binary_string(
            vec!["00", "01"].into_iter().map(str::to_owned).collect(),
        );

        assert_ne!(out, "00");
        assert_ne!(out, "01");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn example_three() {
        let out = Solution::find_different_binary_string(
            vec!["111", "011", "001"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        );

        assert_ne!(out, "111");
        assert_ne!(out, "011");
        assert_ne!(out, "001");
        assert_eq!(out.len(), 3);
    }
}
