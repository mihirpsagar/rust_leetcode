use std::collections::HashMap;

// Time taken: 22:10, 22:16 -> Acc
struct Solution {}

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut result = Vec::new();

        for item in items1 {
            if let Some(&prev) = map.get(&item[0]) {
                map.insert(item[0], prev + item[1]);
            } else {
                map.insert(item[0], item[1]);
            }
        }

        for item in items2 {
            if let Some(&prev) = map.get(&item[0]) {
                map.insert(item[0], prev + item[1]);
            } else {
                map.insert(item[0], item[1]);
            }
        }

        for (key, val) in map {
            Self::binary_insert(&mut result, key, val);
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<Vec<i32>>, key: i32, val: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid][0] < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        arr.insert(left, vec![key, val]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            ),
            [[1, 6], [3, 9], [4, 5]]
        );
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            [[1, 4], [2, 4], [3, 4]]
        );
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 3], vec![2, 2]],
                vec![vec![7, 1], vec![2, 2], vec![1, 4]]
            ),
            [[1, 7], [2, 4], [7, 1]]
        );
    }
}
