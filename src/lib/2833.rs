// Time taken: 23:09, 23:12 -> Acc
struct Solution {}

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut left_count: i32 = 0;
        let mut right_count: i32 = 0;
        let mut remaining_count = 0;

        for ch in moves.chars() {
            if ch == 'L' {
                left_count += 1;
            } else if ch == 'R' {
                right_count += 1;
            } else {
                remaining_count += 1;
            }
        }

        return left_count.abs_diff(right_count) as i32 + remaining_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::furthest_distance_from_origin("L_RL__R".to_string()),
            3
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("_R__LL_".to_string()),
            5
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("_______".to_string()),
            7
        );
    }
}
