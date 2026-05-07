// https://leetcode.com/problems/water-bottles-ii

pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut full_bottles = num_bottles;
        let mut empty_bottles = 0i32;
        let mut num_exchange = num_exchange;
        let mut result = 0;

        loop {
            if full_bottles > 0 {
                result += full_bottles;
                empty_bottles += full_bottles;
                full_bottles = 0;
            } else {
                if num_exchange <= empty_bottles {
                    full_bottles += 1;
                    empty_bottles -= num_exchange;
                    num_exchange += 1;
                } else {
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
