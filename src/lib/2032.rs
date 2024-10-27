use std::collections::HashSet;

// Time taken: 18:55, 19:00 -> Acc
struct Solution {}

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut result = HashSet::new();

        for &num in nums1.iter() {
            set.insert(num);
        }

        for &num in nums2.iter() {
            if set.contains(&num) {
                result.insert(num);
            }
        }

        for &num in nums2.iter() {
            set.insert(num);
        }

        for &num in nums3.iter() {
            if set.contains(&num) {
                result.insert(num);
            }
        }

        return Vec::from_iter(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
            [3, 2]
        );
        assert_eq!(
            Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
            [2, 3, 1]
        );
        assert_eq!(
            Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]),
            []
        );
    }
}
