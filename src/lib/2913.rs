use std::collections::HashSet;

// Time taken: 15:28, 15:35 -> Acc, 15:44 -> Optimized
struct Solution {}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut set = HashSet::new();
            for j in i..nums.len() {
                set.insert(nums[j]);
                result = result + (set.len() * set.len()) as i32;
            }
        }

        return result;
    }

    // pub fn sum_counts(nums: Vec<i32>) -> i32 {
    //     let len = nums.len();
    //     let mut result = 0;
    //     let mut idx1 = 0;

    //     while idx1 < len {
    //         let mut idx2 = idx1 + 1;

    //         while idx2 <= len {
    //             let val = Self::get_distinct_count(&nums, idx1, idx2);
    //             result = result + (val * val);

    //             idx2 += 1;
    //         }

    //         idx1 += 1;
    //     }

    //     return result;
    // }

    // pub fn get_distinct_count(arr: &Vec<i32>, left: usize, right: usize) -> i32 {
    //     let mut set = HashSet::new();
    //     let mut idx = left;

    //     while idx < right {
    //         set.insert(arr[idx]);
    //         idx += 1;
    //     }

    //     return set.len() as i32;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
        assert_eq!(Solution::sum_counts(vec![1, 1]), 3);
    }
}
