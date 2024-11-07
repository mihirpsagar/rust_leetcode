use std::collections::HashMap;

// Time taken: 03:54, 04:04 -> Wrong, 04:15 -> Acc, 04:30 -> Optimized
struct Solution {}

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let len = nums.len();
        let mut result = 0;

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut left_sum = 0;
        let mut right_sum = len as i32;

        for &val in map.values() {
            right_sum -= val;
            result = result + (left_sum * val * right_sum);
            left_sum += val;
        }

        return result;
    }

    // pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    //     let mut result = 0;
    //     let len = nums.len();
    //     let mut idx1 = 0;

    //     while idx1 < len {
    //         let mut idx2 = idx1 + 1;
    //         'inner: while idx2 < len {
    //             if nums[idx1] == nums[idx2] {
    //                 idx2 += 1;
    //                 continue 'inner;
    //             }

    //             let mut idx3 = idx2 + 1;
    //             'inner2: while idx3 < len {
    //                 if nums[idx3] == nums[idx1] || nums[idx3] == nums[idx2] {
    //                     idx3 += 1;
    //                     continue 'inner2;
    //                 }

    //                 result += 1;
    //                 idx3 += 1;
    //             }

    //             idx2 += 1;
    //         }

    //         idx1 += 1;
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::unequal_triplets(vec![1, 3, 1, 2, 4]), 7);
    }
}
