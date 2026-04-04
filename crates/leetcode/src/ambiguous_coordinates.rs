// https://leetcode.com/problems/ambiguous-coordinates

pub struct Solution;

impl Solution {
    fn build_forms(s: &str) -> Vec<String> {
        let mut result = Vec::new();

        if s.len() == 1 || !s.starts_with('0') {
            result.push(s.to_owned());
        }

        for split in 1..s.len() {
            let left = &s[..split];
            let right = &s[split..];

            if left.len() > 1 && left.starts_with('0') {
                continue;
            }

            if right.ends_with('0') {
                continue;
            }

            result.push(format!("{}.{}", left, right));
        }

        result
    }

    fn backtrack_splits(input: &str, split: usize, result: &mut Vec<String>) {
        if split >= input.len() {
            return;
        }

        let left = &input[..split];
        let right = &input[split..];

        let left_options = Self::build_forms(left);
        let right_options = Self::build_forms(right);

        for left_part in &left_options {
            for right_part in &right_options {
                result.push(format!("({}, {})", left_part, right_part));
            }
        }

        Self::backtrack_splits(input, split + 1, result);
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let input = s
            .strip_prefix('(')
            .and_then(|d| d.strip_suffix(')'))
            .unwrap();

        let mut result = Vec::new();

        Self::backtrack_splits(input, 1, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut input: Vec<String>) -> Vec<String> {
        input.sort();
        input
    }

    #[test]
    fn example_one() {
        let expected = vec!["(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::ambiguous_coordinates("(123)".to_owned())),
            sorted(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec!["(0.001, 1)", "(0, 0.011)"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::ambiguous_coordinates("(00011)".to_owned())),
            sorted(expected)
        );
    }

    #[test]
    fn example_three() {
        let expected = vec![
            "(0, 123)",
            "(0, 12.3)",
            "(0, 1.23)",
            "(0.1, 23)",
            "(0.1, 2.3)",
            "(0.12, 3)",
        ];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::ambiguous_coordinates("(0123)".to_owned())),
            sorted(expected)
        );
    }
}
