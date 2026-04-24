// https://leetcode.com/problems/min-cost-climbing-stairs

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = Vec::with_capacity(n);
        dp.push(cost[0]);
        dp.push(cost[1]);

        for index in 2..n {
            dp.push(cost[index] + dp[index - 1].min(dp[index - 2]));
        }

        dp[n - 1].min(dp[n - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![
                1, 100, 1, 1, 1, 100, 1, 1, 100, 1
            ]),
            6
        );
    }
}
