// Time taken: 19:02, 19:16, 19:24 -> Acc
struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx = 0;

        seats.sort();
        students.sort();

        while idx < seats.len() {
            result = result + (seats[idx].abs_diff(students[idx]));
            idx += 1;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
        assert_eq!(
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
            7
        );
        assert_eq!(
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
            4
        );
    }
}
