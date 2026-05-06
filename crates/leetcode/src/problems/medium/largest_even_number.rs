// https://leetcode.com/problems/largest-even-number

pub struct Solution;

impl Solution {
    pub fn largest_even(s: String) -> String {
        let mut bytes = s.as_bytes().to_vec();

        for index in (0..bytes.len()).rev() {
            if bytes[index] == b'1' {
                bytes.pop();
            } else {
                break;
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::largest_even("1112".to_owned()),
            "1112".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_even("221".to_owned()), "22".to_owned());
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::largest_even("1".to_owned()), "".to_owned());
    }
}
