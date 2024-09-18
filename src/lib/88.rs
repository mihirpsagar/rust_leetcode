use std::cmp::Ordering;

// Time taken: 19:25, 19:35 -> Acc
struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;

        for idx in (0..(m + n)).rev() {
            match (m, n) {
                (_, 0) => {
                    break;
                }

                (0, _) => {
                    nums1[idx] = nums2[n - 1];
                    n -= 1;
                }

                _ => match nums1[m - 1].cmp(&nums2[n - 1]) {
                    Ordering::Less => {
                        nums1[idx] = nums2[n - 1];
                        n -= 1;
                    }
                    _ => {
                        nums1[idx] = nums1[m - 1];
                        m -= 1;
                    }
                },
            }
        }
    }

    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     if n == 0 {
    //         return;
    //     }

    //     let mut result: Vec<i32> = Vec::new();
    //     let mut idx1: usize = 0;
    //     let mut idx2: usize = 0;

    //     while idx1 < m as usize && idx2 < n as usize {
    //         if nums1[idx1] < nums2[idx2] {
    //             result.push(nums1[idx1]);
    //             idx1 += 1;
    //         } else {
    //             result.push(nums2[idx2]);
    //             idx2 += 1;
    //         }
    //     }

    //     while idx1 < m as usize {
    //         result.push(nums1[idx1]);
    //         idx1 += 1;
    //     }

    //     while idx2 < n as usize {
    //         result.push(nums2[idx2]);
    //         idx2 += 1;
    //     }

    //     for (idx, &val) in result.iter().enumerate() {
    //         nums1[idx] = val;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
