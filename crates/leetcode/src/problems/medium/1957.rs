// https://leetcode.com/problems/delete-characters-to-make-fancy-string

pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut out = String::new();
        let bytes = s.as_bytes();

        let mut left = 0;

        while left < bytes.len() {
            let mut right = left;

            while right < bytes.len() && bytes[right] == bytes[left] {
                right += 1;
            }

            let len = right - left;
            for _ in 0..(len.min(2)) {
                out.push(bytes[left] as char);
            }

            left = right;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_owned()),
            "leetcode".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::make_fancy_string("aaabaaaa".to_owned()),
            "aabaa".to_owned()
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::make_fancy_string("aab".to_owned()),
            "aab".to_owned()
        );
    }
}
