use std::collections::HashMap;

// Time taken: 10:37, 10:44  -> Acc
struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_map = HashMap::new();
        let mut result = Vec::new();

        for num in nums1 {
            match hash_map.get(&num) {
                None => {
                    hash_map.insert(num, 1);
                }
                Some(val) => {
                    hash_map.insert(num, val + 1);
                }
            }
        }

        for num in nums2 {
            match hash_map.get(&num) {
                None => {}
                Some(&val) => {
                    if val > 0 {
                        result.push(num);
                        hash_map.insert(num, val - 1);
                    }
                }
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
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
