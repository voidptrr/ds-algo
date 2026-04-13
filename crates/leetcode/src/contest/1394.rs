// https://leetcode.com/problems/find-lucky-integer-in-an-array

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freqs = HashMap::<i32, i32>::new();

        for n in arr {
            freqs.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut largest = i32::MIN;
        for (k, v) in &freqs {
            if k == v {
                if *k > largest {
                    largest = *k;
                }
            }
        }

        if largest == i32::MIN { -1 } else { largest }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    }
}
