// https://leetcode.com/problems/harshad-number

pub struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut n = x;
        let mut sum = 0;

        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        if x % sum == 0 { sum } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }
}
