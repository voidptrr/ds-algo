// https://leetcode.com/problems/count-hills-and-valleys-in-an-array

pub struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut count = 0;
        let mut no_dup = Vec::new();
        for n in nums {
            if no_dup.last().copied() != Some(n) {
                no_dup.push(n);
            }
        }

        for index in 1..no_dup.len() - 1 {
            let curr = no_dup[index];
            let prev = no_dup[index - 1];
            let next = no_dup[index + 1];

            if (prev < curr && curr > next) || (prev > curr && curr < next) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }
}
