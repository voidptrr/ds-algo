// https://leetcode.com/problems/finding-the-users-active-minutes

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn finding_users_active_minutes(
        logs: Vec<Vec<i32>>,
        k: i32,
    ) -> Vec<i32> {
        let mut result = vec![0; k as usize];
        let mut user_id_to_times = HashMap::<i32, HashSet<i32>>::new();

        for user_info in logs {
            let user_id = user_info[0];
            let time = user_info[1];

            user_id_to_times.entry(user_id).or_default().insert(time);
        }

        for v in user_id_to_times.values() {
            result[v.len() - 1] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::finding_users_active_minutes(
                vec![
                    vec![0, 5],
                    vec![1, 2],
                    vec![0, 2],
                    vec![0, 5],
                    vec![1, 3]
                ],
                5,
            ),
            vec![0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::finding_users_active_minutes(
                vec![vec![1, 1], vec![2, 2], vec![2, 3]],
                4,
            ),
            vec![1, 1, 0, 0]
        );
    }
}
