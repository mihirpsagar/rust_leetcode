use std::collections::{HashMap, HashSet};

// Time taken: 22:19, 22:24 -> Acc, 21:35 -> Optimized
struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut result = false;
        let k = k as usize;
        let mut hash_set: HashSet<i32> = HashSet::with_capacity(k + 1);

        for (idx, &num) in nums.iter().enumerate() {
            if !hash_set.insert(num) {
                result = true;
                break;
            }
            if idx + 1 > k {
                hash_set.remove(&nums[idx - k]);
            }
        }

        return result;
    }

    // pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    //     let mut result = false;
    //     let mut hash_map: HashMap<i32, usize> = HashMap::new();

    //     for (idx, &num) in nums.iter().enumerate() {
    //         match hash_map.get(&num) {
    //             None => {
    //                 hash_map.insert(num, idx);
    //             }
    //             Some(prev_index) => {
    //                 if idx - prev_index <= k as usize {
    //                     result = true;
    //                     break;
    //                 } else {
    //                     hash_map.insert(num, idx);
    //                 }
    //             }
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
