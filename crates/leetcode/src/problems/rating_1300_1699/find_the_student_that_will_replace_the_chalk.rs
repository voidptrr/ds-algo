// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk

pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|&x| x as i64).sum();
        let mut c = k as i64 % sum;

        let mut result = 0;
        for (index, &chalk) in chalk.iter().enumerate() {
            if c < chalk as i64 {
                result = index as i64;
                break;
            } else {
                c -= chalk as i64;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }
}
