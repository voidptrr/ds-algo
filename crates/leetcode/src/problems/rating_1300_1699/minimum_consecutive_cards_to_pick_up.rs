// https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut prev_map: HashMap<i32, usize> =
            HashMap::with_capacity(cards.len());
        let n = cards.len();
        let mut result = n + 1;

        for (index, card) in cards.iter().enumerate() {
            if let Some(&prev_index) = prev_map.get(card) {
                let diff = index - prev_index + 1;
                result = result.min(diff);
            }

            prev_map.insert(*card, index);
        }

        if result == n + 1 { -1 } else { result as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::minimum_card_pickup(vec![1, 0, 5, 3]), -1);
    }
}
