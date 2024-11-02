use std::collections::HashMap;

// Time taken: 00:17, 00:20, 00:22 -> Acc
struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for &val in map.values() {
            if val % 2 != 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]), true);
        assert_eq!(Solution::divide_array(vec![1, 2, 3, 4]), false);
    }
}
