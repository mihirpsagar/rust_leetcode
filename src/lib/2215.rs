use std::collections::HashSet;

// Time taken: 00:31, 00:34 -> Acc
struct Solution {}

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut result = Vec::new();

        for num in nums1 {
            set1.insert(num);
        }

        for num in nums2 {
            set2.insert(num);
        }

        result.push(vec![]);
        result.push(vec![]);

        for &num in set1.iter() {
            if !set2.contains(&num) {
                result[0].push(num);
            }
        }

        for &num in set2.iter() {
            if !set1.contains(&num) {
                result[1].push(num);
            }
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
            Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            [[1, 3], [4, 6]]
        );
    }
}
