// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity

pub struct Solution;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let digits = num.to_string().into_bytes();

        let mut freq = [0i32; 10];
        for &b in &digits {
            freq[(b - b'0') as usize] += 1;
        }

        let mut next_even = 8usize;
        let mut next_odd = 9usize;
        let mut result = 0i32;

        for &b in &digits {
            let digit = (b - b'0') as usize;
            let chosen = if digit.is_multiple_of(2) {
                while next_even > 0 && freq[next_even] == 0 {
                    next_even -= 2;
                }
                next_even
            } else {
                while next_odd > 0 && freq[next_odd] == 0 {
                    next_odd -= 2;
                }
                next_odd
            };

            freq[chosen] -= 1;
            result = result * 10 + chosen as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::largest_integer(1234), 3412);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}
