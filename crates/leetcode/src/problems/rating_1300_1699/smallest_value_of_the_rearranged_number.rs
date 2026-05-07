// https://leetcode.com/problems/smallest-value-of-the-rearranged-number

pub struct Solution;

impl Solution {
    fn counting_sort(input: &[i64], result: &mut [i64]) {
        if input.is_empty() {
            return;
        }

        let max = *input.iter().max().unwrap() as usize;
        let mut freq = vec![0usize; max + 1];

        for &digit in input {
            freq[digit as usize] += 1;
        }

        for i in 1..freq.len() {
            freq[i] += freq[i - 1];
        }

        for &digit in input.iter().rev() {
            let d = digit as usize;
            freq[d] -= 1;
            result[freq[d]] = digit;
        }
    }

    pub fn smallest_number(num: i64) -> i64 {
        if num == 0 {
            return 0;
        }

        let mut digits = Vec::new();

        let mut num_mut = num;
        let is_negative = num < 0;
        if is_negative {
            num_mut *= -1;
        }

        while num_mut > 0 {
            let d = num_mut % 10;
            digits.push(d);
            num_mut /= 10;
        }

        let mut result = vec![0; digits.len()];
        Self::counting_sort(&digits, &mut result);

        if is_negative {
            result.reverse();
        } else if let Some(idx) = result.iter().position(|&d| d != 0) {
            result.swap(0, idx);
        }
        let n: i64 = result.into_iter().fold(0i64, |acc, d| acc * 10 + d);
        if is_negative { -n } else { n }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::smallest_number(310), 103);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::smallest_number(-7605), -7650);
    }
}
