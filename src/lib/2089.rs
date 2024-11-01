// Time taken: 11:26, 11:42 -> Acc, 11:47 -> Optimized
struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut count_target = 0;
        let mut less_count = 0;
        let mut result = Vec::new();

        for num in nums {
            if num < target {
                less_count += 1;
            } else {
                if num == target {
                    count_target += 1;
                }
            }
        }

        for idx in less_count..less_count + count_target {
            result.push(idx);
        }

        return result;
    }

    // pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let len = nums.len();
    //     let mut result = Vec::new();
    //     Self::quick_sort(&mut nums, 0, len);

    //     for (idx, &num) in nums.iter().enumerate() {
    //         if num == target {
    //             result.push(idx as i32);
    //         }
    //     }

    //     return result;
    // }

    // pub fn quick_sort(mut arr: &mut Vec<i32>, start: usize, end: usize) {
    //     if start >= end {
    //         return;
    //     }

    //     let mut left = start;
    //     let pivot = end - 1;
    //     let mut idx = start;

    //     while idx < pivot {
    //         if arr[idx] < arr[pivot] {
    //             let tmp = arr[left];
    //             arr[left] = arr[idx];
    //             arr[idx] = tmp;
    //             left += 1;
    //         }
    //         idx += 1;
    //     }

    //     let tmp = arr[left];
    //     arr[left] = arr[pivot];
    //     arr[pivot] = tmp;

    //     Self::quick_sort(&mut arr, start, left);
    //     Self::quick_sort(&mut arr, left + 1, end);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
