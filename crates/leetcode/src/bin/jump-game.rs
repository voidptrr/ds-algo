// https://leetcode.com/problems/jump-game

struct Solution;

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

fn main() {
    println!("{}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
}
