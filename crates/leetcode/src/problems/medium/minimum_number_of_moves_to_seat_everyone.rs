// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone

pub struct Solution;

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats_sorted = seats.clone();
        seats_sorted.sort();

        let mut students_sorted = students.clone();
        students_sorted.sort();

        let len = seats.len();

        let mut moves = 0;
        for index in 0..len {
            let seat_pos = seats_sorted[index];
            let student_pos = students_sorted[index];

            if seat_pos == student_pos {
                continue;
            } else if student_pos > seat_pos {
                moves += student_pos - seat_pos;
            } else {
                moves += seat_pos - student_pos;
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
            7
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
            4
        );
    }
}
