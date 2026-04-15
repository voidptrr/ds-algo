// https://leetcode.com/problems/split-with-minimum-sum

pub struct Solution;

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut digits = Vec::<i32>::new();
        let mut x = num;
        while x > 0 {
            let digit = x % 10;
            digits.push(digit);
            x /= 10;
        }

        digits.sort();
        let mut number_1 = 0;
        let mut number_2 = 0;

        for (index, n) in digits.iter().enumerate() {
            if index % 2 == 0 {
                number_1 = number_1 * 10 + n;
            } else {
                number_2 = number_2 * 10 + n;
            }
        }

        number_1 + number_2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::split_num(4325), 59);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::split_num(687), 75);
    }
}
