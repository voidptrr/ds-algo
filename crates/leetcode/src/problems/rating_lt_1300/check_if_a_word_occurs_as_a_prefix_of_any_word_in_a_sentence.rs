// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence

pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (index, word) in sentence.split(' ').enumerate() {
            if word.starts_with(&search_word) {
                return (index + 1) as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::is_prefix_of_word(
                "i love eating burger".to_owned(),
                "burg".to_owned()
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_owned(),
                "pro".to_owned()
            ),
            2
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::is_prefix_of_word(
                "i am tired".to_owned(),
                "you".to_owned()
            ),
            -1
        );
    }
}
