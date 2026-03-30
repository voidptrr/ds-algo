// https://leetcode.com/problems/jump-game

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0usize;

        for (index, jump) in nums.iter().enumerate() {
            if index > farthest {
                return false;
            }

            let next_index = index + (*jump as usize);
            if next_index > farthest {
                farthest = next_index;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_true() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_false() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
