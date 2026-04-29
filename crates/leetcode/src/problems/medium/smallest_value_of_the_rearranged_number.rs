// https://leetcode.com/problems/smallest-value-of-the-rearranged-number

pub struct Solution;

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut digits = Vec::new();

        let mut num_mut = num;
        let is_negative = num < 0;
        if is_negative {
            num_mut *= -1;
        }

        while num_mut > 0 {
            let d = num_mut % 10;
            digits.push(d);
            num_mut /= 10;
        }

        digits.sort_by(|a, b| if is_negative { b.cmp(a) } else { a.cmp(b) });

        if !is_negative && let Some(idx) = digits.iter().position(|&d| d != 0) {
            digits.swap(0, idx);
        }
        let n: i64 = digits.into_iter().fold(0i64, |acc, d| acc * 10 + d);
        if is_negative { -n } else { n }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::smallest_number(310), 103);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::smallest_number(-7605), -7650);
    }
}
