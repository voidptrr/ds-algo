// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks

pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut result = i32::MAX;
        let mut left = 0;
        let mut right = 0;
        let mut ops = 0;

        let chars = blocks.as_bytes();

        while right < blocks.len() {
            if chars[right] == b'W' {
                ops += 1;
            }

            if right - left + 1 > k as usize {
                if chars[left] == b'W' {
                    ops -= 1;
                }

                left += 1;
            }

            if right - left + 1 == k as usize {
                result = result.min(ops);
            }

            right += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_owned(), 7), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_owned(), 2), 0);
    }
}
