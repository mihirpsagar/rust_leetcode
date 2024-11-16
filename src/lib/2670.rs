use std::collections::HashMap;

// Time taken: 11:52, 11:59 -> Acc
struct Solution {}

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut suffix_map = HashMap::new();
        let mut prefix_map = HashMap::new();
        let mut result = Vec::new();

        for num in nums.iter() {
            *suffix_map.entry(*num).or_insert(0) += 1;
        }

        for num in nums {
            *prefix_map.entry(num).or_insert(0) += 1;
            if let Some(val) = suffix_map.get_mut(&num) {
                if *val == 1 {
                    suffix_map.remove(&num);
                } else {
                    *val -= 1;
                }
            }
            result.push(prefix_map.len() as i32 - suffix_map.len() as i32);
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
            Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
            [-3, -1, 1, 3, 5]
        );
        assert_eq!(
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
            [-2, -1, 0, 2, 3]
        );
    }
}
