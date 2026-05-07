// https://leetcode.com/problems/increasing-decreasing-string

pub struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let n = s.len();
        let mut freq = [0usize; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut out = String::with_capacity(n);

        while out.len() < n {
            for (i, count) in freq.iter_mut().enumerate() {
                if *count > 0 {
                    out.push((b'a' + i as u8) as char);
                    *count -= 1;
                }
            }

            for (i, count) in freq.iter_mut().enumerate().rev() {
                if *count > 0 {
                    out.push((b'a' + i as u8) as char);
                    *count -= 1;
                }
            }
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
            Solution::sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::sort_string("rat".to_string()), "art");
    }
}
