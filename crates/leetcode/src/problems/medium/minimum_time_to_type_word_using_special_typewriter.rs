// https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter

pub struct Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut count = 0;
        let mut prev = b'a';

        for &b in word.as_bytes() {
            let prev_idx = (prev - b'a') as i32;
            let curr_idx = (b - b'a') as i32;
            let distance = (curr_idx - prev_idx).abs();

            let d = distance.min(26 - distance);
            count += 1 + d;
            prev = b;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_time_to_type("abc".to_owned()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::min_time_to_type("bza".to_owned()), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::min_time_to_type("zjpc".to_owned()), 34);
    }
}
