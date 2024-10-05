// Time taken: 00:20, 00:37 -> Acc, 00:54 -> Optimized
struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }

        let target = sum / 3;
        let mut idx = 0;
        let mut curr_sum = 0;
        let mut partition = 0;

        while idx < arr.len() - 1 {
            curr_sum += arr[idx];
            if curr_sum == target {
                partition += 1;
                if partition == 2 {
                    return true;
                }
                curr_sum = 0;
            }
            idx += 1;
        }

        return partition >= 2;
    }
    // pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    //     let mut left_sum = vec![0; arr.len()];
    //     let mut right_sum = vec![0; arr.len()];

    //     let mut idx = 0;
    //     let mut sum = 0;
    //     while idx < arr.len() {
    //         sum += arr[idx];
    //         left_sum[idx] = sum;
    //         idx += 1;
    //     }

    //     let mut idx = arr.len() - 1;
    //     let mut sum = 0;
    //     loop {
    //         sum += arr[idx];
    //         right_sum[idx] = sum;

    //         if idx == 0 {
    //             break;
    //         }
    //         idx -= 1;
    //     }

    //     let mut idx1 = 0;
    //     while idx1 < left_sum.len() {
    //         let mut idx2 = idx1 + 2;
    //         while idx2 < right_sum.len() {
    //             if left_sum[idx1] == right_sum[idx2] {
    //                 if (left_sum[idx2 - 1] - left_sum[idx1]) == right_sum[idx2] {
    //                     return true;
    //                 }
    //             }
    //             idx2 += 1;
    //         }
    //         idx1 += 1;
    //     }

    //     return false;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
            true
        );
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
            false
        );
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
            true
        );
    }
}
