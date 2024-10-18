use std::collections::HashMap;

// Time taken: 15:10, 15:25 -> Acc
struct Solution {}

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut arr = Vec::new();
        let mut result = Vec::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for (key, val) in map {
            Self::binary_insert(&mut arr, (key, val));
        }

        for val in arr {
            for _ in 0..val.1 {
                result.push(val.0);
            }
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<(i32, i32)>, target: (i32, i32)) {
        if arr.len() == 0 {
            arr.push(target);
            return;
        }

        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid].1 < target.1 {
                left = mid + 1;
            } else if arr[mid].1 > target.1 {
                right = mid;
            } else {
                if arr[mid].0 > target.0 {
                    left = mid + 1;
                } else {
                    right = mid;
                }
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
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            [3, 1, 1, 2, 2, 2]
        );
        assert_eq!(
            Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
            [1, 3, 3, 2, 2]
        );
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            [5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
