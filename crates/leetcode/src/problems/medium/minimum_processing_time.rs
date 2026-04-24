// https://leetcode.com/problems/minimum-processing-time

pub struct Solution;

impl Solution {
    pub fn min_processing_time(
        processor_time: Vec<i32>,
        tasks: Vec<i32>,
    ) -> i32 {
        let mut result = 0;

        let mut processor_time_mut = processor_time.clone();
        processor_time_mut.sort();

        let mut tasks_time = tasks.clone();
        tasks_time.sort_unstable_by(|a, b| b.cmp(a));

        for index in 0..processor_time_mut.len() {
            let total_time = processor_time_mut[index] + tasks_time[index * 4];
            result = result.max(total_time);
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
            Solution::min_processing_time(
                vec![8, 10],
                vec![2, 2, 3, 1, 8, 7, 4, 5]
            ),
            16
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_processing_time(
                vec![10, 20],
                vec![2, 3, 1, 2, 5, 8, 4, 3]
            ),
            23
        );
    }
}
