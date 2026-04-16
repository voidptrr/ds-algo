// https://leetcode.com/problems/shifting-letters

pub struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let bytes = s.as_bytes();
        let mut out = vec![0u8; bytes.len()];

        let mut total = 0;
        for i in (0..shifts.len()).rev() {
            total = (total + shifts[i] as i64) % 26;

            let base = (bytes[i] - b'a') as i64;
            let shifted = (base + total) % 26;
            out[i] = b'a' + shifted as u8;
        }

        String::from_utf8(out).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::shifting_letters("abc".to_owned(), vec![3, 5, 9]),
            "rpl".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::shifting_letters("aaa".to_owned(), vec![1, 2, 3]),
            "gfd".to_owned()
        );
    }
}
