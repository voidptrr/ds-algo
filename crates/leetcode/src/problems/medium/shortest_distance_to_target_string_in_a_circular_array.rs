// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array

pub struct Solution;

impl Solution {
    pub fn closest_target(
        words: Vec<String>,
        target: String,
        start_index: i32,
    ) -> i32 {
        let mut positions = Vec::new();
        for (index, w) in words.iter().enumerate() {
            if *w == target {
                positions.push(index as i32);
            }
        }

        let mut min = i32::MAX;

        for pos in &positions {
            let d = (pos - start_index).abs();
            let min_distance = d.min(words.len() as i32 - d);

            min = min.min(min_distance);
        }

        if min == i32::MAX { -1 } else { min }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::closest_target(
                vec!["hello", "i", "am", "leetcode", "hello"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                "hello".to_owned(),
                1,
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::closest_target(
                vec!["a", "b", "leetcode"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                "leetcode".to_owned(),
                0,
            ),
            1
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::closest_target(
                vec!["i", "eat", "leetcode"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                "ate".to_owned(),
                0,
            ),
            -1
        );
    }
}
