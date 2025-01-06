// Time taken: 18:29, 18:32 -> Acc, 18:41, 19:00 -> Optimized
struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let missing = arr[mid] - (mid as i32 + 1);

            if missing < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == 0 {
            return k;
        }

        left = left - 1;

        return k + left as i32 + 1;
    }

    // pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    //     let mut left = 0;
    //     let mut k = k;
    //     let mut idx = 0;

    //     while idx < arr.len() {
    //         if arr[idx] - left - 1 != 0 {
    //             let val = arr[idx] - left - 1;
    //             if val < k {
    //                 k -= val;
    //             } else {
    //                 return left + k;
    //             }
    //         }
    //         left = arr[idx];
    //         idx += 1;
    //     }

    //     return left + k;
    // }

    // pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    //     let mut idx = 0;
    //     let mut missing = 1;
    //     let mut k = k;

    //     while idx < arr.len() {
    //         if arr[idx] == missing {
    //             missing += 1;
    //             idx += 1;
    //         } else {
    //             k -= 1;
    //             if k == 0 {
    //                 return missing;
    //             }
    //             missing += 1;
    //         }
    //     }

    //     if k != 0 {
    //         missing += k - 1;
    //     }

    //     return missing;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
