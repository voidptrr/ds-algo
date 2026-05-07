// https://leetcode.com/problems/generate-parentheses

pub struct Solution;

impl Solution {
    fn backtrack(
        n: i32,
        open_p: i32,
        close_p: i32,
        cand: &mut String,
        result: &mut Vec<String>,
    ) {
        if open_p == close_p && open_p + close_p == n * 2 {
            result.push(cand.clone());
            return;
        }

        if open_p < n {
            cand.push('(');
            Self::backtrack(n, open_p + 1, close_p, cand, result);
            cand.pop();
        }

        if close_p < open_p {
            cand.push(')');
            Self::backtrack(n, open_p, close_p + 1, cand, result);
            cand.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut cand = String::with_capacity((n * 2) as usize);

        Self::backtrack(n, 0, 0, &mut cand, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utils::sorted_strings;

    #[test]
    fn example_one() {
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted_strings(Solution::generate_parenthesis(3)),
            sorted_strings(expected)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
