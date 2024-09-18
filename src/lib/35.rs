use std::cmp::Ordering;

// Time taken: 14:36, 14:40 -> Wrong, 14:47 -> Acc
struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return nums.binary_search(&target).unwrap_or_else(|idx| idx) as i32;
    }

    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut left: usize = 0;
    //     let mut right: usize = nums.len();
    //     let mut mid: usize = 0;

    //     while left < right {
    //         mid = left + ((right - left) / 2);
    //         match target.cmp(&nums[mid]) {
    //             Ordering::Equal => {
    //                 break;
    //             }
    //             Ordering::Less => {
    //                 right = mid;
    //             }
    //             Ordering::Greater => {
    //                 left = mid + 1;
    //             }
    //         }
    //     }

    //     if mid < nums.len() {
    //         if target > nums[mid] {
    //             mid += 1;
    //         }
    //     }

    //     return mid as i32;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
