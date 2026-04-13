// https://leetcode.com/problems/defanging-an-ip-address

pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_owned()),
            "1[.]1[.]1[.]1".to_owned()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::defang_i_paddr("255.100.50.0".to_owned()),
            "255[.]100[.]50[.]0".to_owned()
        );
    }
}
