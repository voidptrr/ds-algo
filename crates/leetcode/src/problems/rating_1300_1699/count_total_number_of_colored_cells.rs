// https://leetcode.com/problems/count-total-number-of-colored-cells

pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n_ = n as i64;
        2 * n_.pow(2) - 2 * n_ + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::colored_cells(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::colored_cells(2), 5);
    }
}
