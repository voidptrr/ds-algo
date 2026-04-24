// https://leetcode.com/problems/optimal-partition-of-string

pub struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut substring = String::new();
        let mut result = Vec::<String>::new();

        for c in s.chars() {
            if !substring.contains(c) {
                substring.push(c);
            } else {
                result.push(substring.to_owned());
                substring.clear();
                substring.push(c);
            }
        }

        if !substring.is_empty() {
            result.push(substring);
        }

        result.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::partition_string("abacaba".to_owned()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::partition_string("ssssss".to_owned()), 6);
    }
}
