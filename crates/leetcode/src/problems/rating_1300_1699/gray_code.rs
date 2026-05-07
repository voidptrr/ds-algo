// https://leetcode.com/problems/gray-code

pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let bits = u32::try_from(n).expect("n must be non-negative");
        let total = 1usize << bits;
        let mut result = Vec::with_capacity(total);

        for i in 0..total {
            result.push((i ^ (i >> 1)) as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let expected = vec![0, 1, 3, 2];
        assert_eq!(Solution::gray_code(2), expected);
    }

    #[test]
    fn example_two() {
        let expected = vec![0, 1];
        assert_eq!(Solution::gray_code(1), expected);
    }
}
