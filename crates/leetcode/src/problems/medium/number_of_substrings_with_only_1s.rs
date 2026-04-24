// https://leetcode.com/problems/number-of-substrings-with-only-1s

pub struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut ones: i64 = 0;
        let mut total_sub: i64 = 0;
        for c in s.chars() {
            if c == '0' {
                if ones > 0 {
                    total_sub += ones * (ones + 1) / 2;
                    ones = 0;
                }
            } else if c == '1' {
                ones += 1;
            }
        }

        if ones > 0 {
            total_sub += ones * (ones + 1) / 2;
        }

        (total_sub % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::num_sub("0110111".to_owned()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::num_sub("101".to_owned()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::num_sub("111111".to_owned()), 21);
    }
}
