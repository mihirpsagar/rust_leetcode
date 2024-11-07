use std::collections::HashSet;

// Time taken: 03:37, 03:45 -> Acc
struct Solution {}

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let len = nums.len();

        Self::quick_sort(&mut nums, 0, len);

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            set.insert(nums[left] + nums[right]);
            left += 1;
            right -= 1;
        }

        if left == right {
            set.insert(nums[left]);
        }

        return set.len() as i32;
    }

    pub fn quick_sort(mut arr: &mut Vec<i32>, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mut left = start;
        let mut idx = start;
        let pivot = end - 1;

        while idx < end {
            if arr[idx] < arr[pivot] {
                let tmp = arr[left];
                arr[left] = arr[idx];
                arr[idx] = tmp;
                left += 1;
            }
            idx += 1;
        }

        let tmp = arr[left];
        arr[left] = arr[pivot];
        arr[pivot] = tmp;

        Self::quick_sort(&mut arr, start, left);
        Self::quick_sort(&mut arr, left + 1, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
        assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
    }
}
