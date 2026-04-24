// https://leetcode.com/problems/best-reachable-tower

pub struct Solution;

impl Solution {
    #[inline]
    fn is_reachable(x: i32, y: i32, cx: i32, cy: i32, r: i32) -> bool {
        let manhattan_distance = (x - cx).abs() + (y - cy).abs();
        manhattan_distance <= r
    }

    pub fn best_tower(
        towers: Vec<Vec<i32>>,
        center: Vec<i32>,
        radius: i32,
    ) -> Vec<i32> {
        let mut best_coord = (i32::MIN, i32::MIN);
        let mut best_quality = i32::MIN;
        let mut found = false;

        let cx = center[0];
        let cy = center[1];

        for coord in towers {
            let x = coord[0];
            let y = coord[1];
            let q = coord[2];
            if Self::is_reachable(x, y, cx, cy, radius) {
                if q > best_quality
                    || (q == best_quality && ((x, y) < best_coord))
                {
                    best_quality = q;
                    best_coord = (x, y);
                }

                found = true;
            }
        }

        if found {
            vec![best_coord.0, best_coord.1]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]],
                vec![1, 1],
                2
            ),
            vec![3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]],
                vec![0, 0],
                5
            ),
            vec![1, 3]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![5, 6, 8], vec![0, 3, 5]],
                vec![1, 2],
                1
            ),
            vec![-1, -1]
        );
    }
}
