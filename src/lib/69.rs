use std::cmp::{self, Ordering};

// Time taken: 21:04, 21:13 -> Wrong, 21:31 -> Acc, 21:36 -> Optimization
struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        const MAX: i32 = 46340;
        let mut left = 0;
        let mut right = cmp::min(x, MAX);

        while left <= right {
            let mid = left + ((right - left) / 2);
            let val = mid * mid;

            match val.cmp(&x) {
                Ordering::Equal => {
                    return mid;
                }
                Ordering::Greater => {
                    right = mid - 1;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        return right;
    }

    // pub fn my_sqrt(x: i32) -> i32 {
    //     let mut result = 0;
    //     let mut left = 0;
    //     let mut right = x / 2;
    //     let mut found = false;

    //     if x == 1 {
    //         return 1;
    //     }

    //     while left < right {
    //         let mid = left + ((right - left) / 2);
    //         let val = match mid.checked_mul(mid) {
    //             None => {
    //                 right = mid;
    //                 continue;
    //             }
    //             Some(res) => res,
    //         };

    //         match val.cmp(&x) {
    //             Ordering::Equal => {
    //                 result = mid;
    //                 found = true;
    //                 break;
    //             }
    //             Ordering::Greater => {
    //                 right = mid;
    //             }
    //             Ordering::Less => {
    //                 left = mid + 1;
    //             }
    //         }
    //     }

    //     if !found {
    //         match left.checked_mul(left) {
    //             None => {
    //                 result = left - 1;
    //             }
    //             Some(res) => {
    //                 if res > x {
    //                     result = left - 1;
    //                 } else {
    //                     result = left;
    //                 }
    //             }
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }
}
