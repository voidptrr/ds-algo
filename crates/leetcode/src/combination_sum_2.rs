// https://leetcode.com/problems/combination-sum-ii

pub struct Solution;

impl Solution {
    fn backtrack(
        start: usize,
        input: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
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
            if index > start && input[index] == input[index - 1] {
                continue;
            }

            path.push(input[index]);
            Self::backtrack(
                index + 1,
                input,
                result,
                path,
                remain - input[index],
            );
            path.pop();
        }
    }

    pub fn combination_sum2(
        candidates: Vec<i32>,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut path = Vec::<i32>::new();

        let mut candidates_mut = candidates.clone();
        candidates_mut.sort();

        Self::backtrack(0, &candidates_mut, &mut result, &mut path, target);

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
        let expected =
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(
            sorted(Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)),
            sorted(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec![vec![1, 2, 2], vec![5]];
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            sorted(expected)
        );
    }
}
