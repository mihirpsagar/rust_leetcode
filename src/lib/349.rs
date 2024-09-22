use std::collections::HashSet;

// Time taken: 10:24, 10:28 -> Acc
struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_set = HashSet::new();
        let mut result = HashSet::new();

        for num in nums1 {
            hash_set.insert(num);
        }

        for num in nums2 {
            if hash_set.contains(&num) {
                result.insert(num);
            }
        }

        let result: Vec<i32> = result.iter().map(|&x| x).collect();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
