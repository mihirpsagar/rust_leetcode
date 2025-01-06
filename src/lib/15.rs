use std::collections::HashSet;

// Time taken: 16:15, 16:26, 16:39 -> TLE, 17:41 -> Acc, 17:59 -> Optimized
struct Solution {}

impl Solution {
    // Sort & Two pointer -> 19 ms
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut seen_set: HashSet<i32> = HashSet::new();
        let mut result_set: HashSet<Vec<i32>> = HashSet::new();
        let len = nums.len();

        nums.sort();

        let mut i = 0;
        while i < len - 2 {
            if seen_set.insert(nums[i]) {
                let mut j = i + 1;
                let mut k = len - 1;
                while j < k {
                    let sum = nums[i] + nums[j] + nums[k];
                    if sum == 0 {
                        let mut vec = vec![nums[i], nums[j], nums[k]];
                        vec.sort();
                        result_set.insert(vec);

                        j += 1;
                        while j < k && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                    } else if sum < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }

            i += 1;
        }

        let mut result = Vec::new();
        for res in result_set {
            result.push(res);
        }

        return result;
    }

    // pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     let mut set = HashSet::new();
    //     nums.sort();

    //     let mut idx1 = 0;
    //     while idx1 < nums.len() - 2 {
    //         let mut left = idx1 + 1;
    //         let mut right = nums.len() - 1;
    //         while left < right {
    //             let sum = nums[idx1] + nums[left] + nums[right];
    //             match sum.cmp(&0) {
    //                 Ordering::Equal => {
    //                     let mut arr = vec![nums[idx1], nums[left], nums[right]];
    //                     arr.sort();
    //                     set.insert(arr);
    //                     left += 1;
    //                     while left < nums.len() && nums[left] == nums[left - 1] {
    //                         left += 1;
    //                     }
    //                 }
    //                 Ordering::Greater => {
    //                     right -= 1;
    //                 }
    //                 Ordering::Less => {
    //                     left += 1;
    //                 }
    //             }
    //         }
    //         idx1 += 1;
    //     }

    //     let mut result = Vec::new();
    //     for arr in set {
    //         result.push(arr);
    //     }
    //     return result;
    // }

    // Second try -> 853 ms
    // pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     let mut result_set: HashSet<Vec<i32>> = HashSet::new();
    //     let mut found_set = HashSet::new();

    //     let mut idx = 0;
    //     while idx < nums.len() {
    //         if found_set.insert(&nums[idx]) {
    //             let two_sum_arr = Self::all_two_sum(&nums, idx);
    //             for val in two_sum_arr {
    //                 let mut vec = vec![nums[idx], val.0, val.1];
    //                 vec.sort();
    //                 result_set.insert(vec);
    //             }
    //         }
    //         idx += 1;
    //     }

    //     let mut result = Vec::new();
    //     for set in result_set {
    //         result.push(set);
    //     }

    //     return result;
    // }

    // pub fn all_two_sum(nums: &Vec<i32>, target: usize) -> Vec<(i32, i32)> {
    //     let mut set: HashSet<i32> = HashSet::new();
    //     let mut result_set: HashSet<(i32, i32)> = HashSet::new();
    //     let target_val = nums[target] * -1;

    //     let mut idx = 0;
    //     while idx < nums.len() {
    //         if idx == target {
    //             idx += 1;
    //             continue;
    //         }

    //         if set.contains(&(target_val - nums[idx])) {
    //             result_set.insert((
    //                 std::cmp::min(nums[idx], target_val - nums[idx]),
    //                 std::cmp::max(nums[idx], target_val - nums[idx]),
    //             ));
    //         }
    //         set.insert(nums[idx]);

    //         idx += 1;
    //     }

    //     let mut result = Vec::new();
    //     for set in result_set {
    //         result.push(set);
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]).is_empty(), true);
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), [[0, 0, 0]]);
    }
}
