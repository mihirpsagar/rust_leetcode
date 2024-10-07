use std::collections::HashMap;

// Time taken: 00:16, 00:28 -> Acc
struct Solution {}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();

        for (idx, &num) in arr2.iter().enumerate() {
            map.insert(num, idx);
        }

        let mut idx = 0;
        while idx < arr1.len() {
            let insert_idx = Self::binary_search(&result, &map, arr1[idx]);
            result.insert(insert_idx, arr1[idx]);
            idx += 1;
        }

        return result;
    }

    fn binary_search(arr: &Vec<i32>, map: &HashMap<i32, usize>, target: i32) -> usize {
        let mut left = 0;
        let mut right = arr.len();
        let target_idx = map.get(&target);

        while left < right {
            let mid = left + (right - left) / 2;
            if let Some(curr_idx) = map.get(&arr[mid]) {
                if let Some(target_idx) = target_idx {
                    if curr_idx <= target_idx {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                } else {
                    left = mid + 1;
                }
            } else {
                if target_idx.is_some() {
                    right = mid;
                } else {
                    if arr[mid] <= target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
            }
        }

        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
        assert_eq!(
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            [22, 28, 8, 6, 17, 44]
        );
    }
}
