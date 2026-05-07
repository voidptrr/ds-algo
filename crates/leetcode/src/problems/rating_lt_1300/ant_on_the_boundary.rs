// https://leetcode.com/problems/ant-on-the-boundary

pub struct Solution;

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut boundary = 0;
        let mut times = 0;

        for n in nums {
            if n < 0 {
                boundary -= -n;
            } else {
                boundary += n;
            }

            if boundary == 0 {
                times += 1;
            }
        }

        times
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
