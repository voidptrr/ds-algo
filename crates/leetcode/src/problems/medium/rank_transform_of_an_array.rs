// https://leetcode.com/problems/rank-transform-of-an-array

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut arr_mut = arr.clone();
        arr_mut.sort();

        let mut val_rank_map = HashMap::<i32, i32>::new();
        let mut result = vec![0; arr.len()];

        let mut rank = 1;
        for n in arr_mut {
            match val_rank_map.get(&n) {
                Some(_) => continue,
                None => {
                    val_rank_map.insert(n, rank);
                    rank += 1;
                }
            }
        }

        for index in 0..result.len() {
            let value = arr[index];
            let rank = val_rank_map.get(&value).unwrap();

            result[index] = *rank;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::array_rank_transform(vec![
                37, 12, 28, 9, 100, 56, 80, 5, 12
            ]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
