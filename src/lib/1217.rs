// Time taken: 00:09, 00:19, 00:28 -> Acc, 00:30 -> Optimized
struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;

        for num in position {
            if num % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }

        return std::cmp::min(odd, even);
    }

    // pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    //     let mut result = None;
    //     let len = position.len();
    //     let mut idx1 = 0;

    //     while idx1 < len {
    //         let mut idx2 = 0;
    //         let mut sum = 0;
    //         while idx2 < len {
    //             if position[idx2] != position[idx1]
    //                 && position[idx2].abs_diff(position[idx1]) % 2 == 1
    //             {
    //                 sum += 1;
    //             }
    //             idx2 += 1;
    //         }
    //         if let Some(prev_val) = result {
    //             if prev_val > sum {
    //                 result = Some(sum);
    //             }
    //         } else {
    //             result = Some(sum);
    //         }
    //         idx1 += 1;
    //     }

    //     if result.is_none() {
    //         return 0;
    //     } else {
    //         return result.unwrap();
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 1000000000]), 1);
    }
}
