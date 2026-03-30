// https://leetcode.com/problems/jump-game-ii

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut current_end = 0usize;
        let mut farthest = 0usize;

        for (index, jump) in
            nums.iter().enumerate().take(nums.len().saturating_sub(1))
        {
            let reach = index + (*jump as usize);
            if reach > farthest {
                farthest = reach;
            }

            if index == current_end {
                jumps += 1;
                current_end = farthest;
            }
        }

        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
