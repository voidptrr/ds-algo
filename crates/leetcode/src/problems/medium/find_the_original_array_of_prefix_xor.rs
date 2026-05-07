// https://leetcode.com/problems/find-the-original-array-of-prefix-xor

pub struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut xor_pref = vec![0i32; pref.len()];
        xor_pref[0] = pref[0];

        for index in 1..pref.len() {
            xor_pref[index] = pref[index - 1] ^ pref[index];
        }

        xor_pref
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}
