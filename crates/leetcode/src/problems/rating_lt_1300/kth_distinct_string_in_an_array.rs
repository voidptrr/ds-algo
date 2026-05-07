// https://leetcode.com/problems/kth-distinct-string-in-an-array

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut count_map = HashMap::<String, (usize, i32)>::new();

        for (index, word) in arr.iter().enumerate() {
            count_map
                .entry(word.to_owned())
                .and_modify(|v| v.1 += 1)
                .or_insert((index, 1));
        }

        let mut distinct: Vec<(usize, String)> = count_map
            .into_iter()
            .filter_map(|(s, (first_i, count))| {
                (count == 1).then_some((first_i, s))
            })
            .collect();

        if distinct.len() < k as usize {
            return "".to_owned();
        }

        distinct.sort_by_key(|(index, _)| *index);
        distinct
            .get((k - 1) as usize)
            .map(|(_, s)| s.clone())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::kth_distinct(
                vec!["d", "b", "c", "b", "c", "a"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                2
            ),
            "a".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::kth_distinct(
                vec!["aaa", "aa", "a"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                1
            ),
            "aaa".to_owned()
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::kth_distinct(
                vec!["a", "b", "a"].into_iter().map(str::to_owned).collect(),
                3
            ),
            "".to_owned()
        );
    }
}
