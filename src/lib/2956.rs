use std::collections::HashSet;

// Time taken: 16:49, 16:53 -> Acc
struct Solution {}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut result = vec![0, 0];

        let mut idx = 0;
        while idx < nums1.len() {
            set1.insert(nums1[idx]);
            idx += 1;
        }

        idx = 0;
        while idx < nums2.len() {
            set2.insert(nums2[idx]);
            if set1.contains(&nums2[idx]) {
                result[1] += 1;
            }
            idx += 1;
        }

        idx = 0;
        while idx < nums1.len() {
            if set2.contains(&nums1[idx]) {
                result[0] += 1;
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_intersection_values(vec![2, 3, 2], vec![1, 2]),
            [2, 1]
        );
        assert_eq!(
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            [3, 4]
        );
        assert_eq!(
            Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
            [0, 0]
        );
    }
}
