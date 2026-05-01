// https://leetcode.com/problems/rearrange-spaces-between-words

pub struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut number_space = 0;
        let mut result = String::new();

        for c in text.chars() {
            if c == ' ' {
                number_space += 1;
            }
        }

        if number_space < 1 {
            return text;
        }

        let mut word_count = text.split_whitespace().count();
        if word_count == 1 {
            word_count += 1;
        }
        let number_spaces_between_words = number_space / (word_count - 1);

        let mut space_count = 0;
        for (index, word) in text.split_whitespace().enumerate() {
            result.push_str(word);
            if index < word_count - 1 {
                result.push_str(&" ".repeat(number_spaces_between_words));
                space_count += number_spaces_between_words;
            }
        }

        if space_count < number_space {
            result.push_str(&" ".repeat(number_space - space_count));
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
            Solution::reorder_spaces("  this   is  a sentence ".to_owned()),
            "this   is   a   sentence".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::reorder_spaces(" practice   makes   perfect".to_owned()),
            "practice   makes   perfect ".to_owned()
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::reorder_spaces("hello   world".to_owned()),
            "hello   world".to_owned()
        );
    }
}
