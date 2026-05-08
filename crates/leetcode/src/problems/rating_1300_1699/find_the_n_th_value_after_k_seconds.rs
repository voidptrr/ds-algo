// https://leetcode.com/problems/find-the-n-th-value-after-k-seconds

pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1_i64;
        base %= Self::MOD;

        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % Self::MOD;
            }
            base = (base * base) % Self::MOD;
            exp >>= 1;
        }

        result
    }

    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        let a = (n + k - 1) as usize;
        let b = (n - 1) as usize;

        let mut fact = vec![1_i64; a + 1];
        for i in 1..=a {
            fact[i] = (fact[i - 1] * i as i64) % Self::MOD;
        }

        let mut inv_fact = vec![1_i64; a + 1];
        inv_fact[a] = Self::mod_pow(fact[a], Self::MOD - 2);
        for i in (1..=a).rev() {
            inv_fact[i - 1] = (inv_fact[i] * i as i64) % Self::MOD;
        }

        let result = (((fact[a] * inv_fact[b]) % Self::MOD) * inv_fact[a - b])
            % Self::MOD;
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
    }
}
