use std::collections::HashSet;

// Time taken: 01:55, 02:01 -> Acc
struct Solution {}

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut set = HashSet::new();
        let mut idx = 1;

        for &val in nums[0].iter() {
            set.insert(val);
        }

        while idx < nums.len() && !set.is_empty() {
            let mut set2 = HashSet::new();
            for &val in nums[idx].iter() {
                if set.contains(&val) {
                    set2.insert(val);
                }
            }
            set = set2;
            idx += 1;
        }

        for val in set {
            Self::binary_insert(&mut result, val);
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<i32>, target: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        arr.insert(left, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            [3, 4]
        );
        assert_eq!(
            Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            []
        );
    }
}
