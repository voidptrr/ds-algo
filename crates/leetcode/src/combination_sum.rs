// https://leetcode.com/problems/combination-sum

pub struct Solution;

impl Solution {
    fn backtrack(
        start: usize,
        input: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        target: i32,
        path: &mut Vec<i32>,
        remain: i32,
    ) {
        if remain == 0 {
            result.push(path.clone());
            return;
        }

        if remain < 0 {
            return;
        }

        for index in start..input.len() {
            path.push(input[index]);
            Self::backtrack(
                index,
                input,
                result,
                target,
                path,
                remain - input[index],
            );
            path.pop();
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut path = Vec::<i32>::new();

        Self::backtrack(0, &candidates, &mut result, target, &mut path, target);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        input.sort();
        input
    }

    #[test]
    fn example_one() {
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(
            sorted(Solution::combination_sum(vec![2, 3, 6, 7], 7)),
            sorted(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            sorted(expected)
        );
    }

    #[test]
    fn example_three() {
        let expected = vec![];
        assert_eq!(Solution::combination_sum(vec![2], 1), sorted(expected));
    }
}
