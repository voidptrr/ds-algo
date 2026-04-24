// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time

pub struct Solution;

impl Solution {
    pub fn busy_student(
        start_time: Vec<i32>,
        end_time: Vec<i32>,
        query_time: i32,
    ) -> i32 {
        let times_len = start_time.len();
        let mut count = 0;

        for index in 0..times_len {
            let is_query_time_between = query_time >= start_time[index]
                && query_time <= end_time[index];
            if is_query_time_between {
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
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    }
}
