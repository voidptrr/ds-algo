// https://leetcode.com/problems/generate-binary-strings-without-adjacent-zeros

pub struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut cand = String::new();

        fn backtrack(
            n: usize,
            result: &mut Vec<String>,
            cand: &mut String,
            last_zero: bool,
        ) {
            if cand.len() == n {
                result.push(cand.to_owned());
                return;
            }

            if !last_zero {
                cand.push('0');
                backtrack(n, result, cand, true);
                cand.pop();
            }

            cand.push('1');
            backtrack(n, result, cand, false);
            cand.pop();
        }

        backtrack(n as usize, &mut result, &mut cand, false);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utils::sorted_strings;

    #[test]
    fn example_one() {
        let expected = vec!["010", "011", "101", "110", "111"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted_strings(Solution::valid_strings(3)),
            sorted_strings(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec!["0", "1"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted_strings(Solution::valid_strings(1)),
            sorted_strings(expected)
        );
    }
}
