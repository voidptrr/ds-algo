// https://leetcode.com/problems/restore-ip-addresses

pub struct Solution;

impl Solution {
    fn backtrack(
        start: usize,
        input: &String,
        result: &mut Vec<String>,
        ip: &mut Vec<String>,
    ) {
        if ip.len() == 4 && start == input.len() {
            let full_ip = ip.join(".");

            if result.contains(&full_ip) {
                return;
            }

            result.push(full_ip);
            return;
        }

        for index in 1..=3 {
            if start + index > input.len() {
                break;
            }

            let seg = &input[start..start + index];
            if seg.len() > 1 && seg.starts_with('0') {
                break;
            }

            let num: i32 = seg.parse().unwrap();
            if num > 255 {
                break;
            }

            ip.push(seg.to_string());
            Self::backtrack(start + index, input, result, ip);
            ip.pop();
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut ip = Vec::<String>::new();

        Self::backtrack(0, &s, &mut result, &mut ip);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut input: Vec<String>) -> Vec<String> {
        input.sort();
        input
    }

    #[test]
    fn example_one() {
        let expected = vec!["255.255.11.135", "255.255.111.35"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::restore_ip_addresses("25525511135".to_owned())),
            sorted(expected)
        );
    }

    #[test]
    fn example_two() {
        let expected = vec!["0.0.0.0"];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::restore_ip_addresses("0000".to_owned())),
            sorted(expected)
        );
    }

    #[test]
    fn example_three() {
        let expected = vec![
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];
        let expected =
            expected.into_iter().map(str::to_owned).collect::<Vec<_>>();

        assert_eq!(
            sorted(Solution::restore_ip_addresses("101023".to_owned())),
            sorted(expected)
        );
    }
}
