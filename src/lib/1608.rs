use std::collections::HashMap;

// Time taken: 12:30, 12:37 -> Acc
struct Solution {}

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let len = nums.len() as i32;

        for num in nums {
            for val in 1..=len {
                if num >= val {
                    *map.entry(val).or_insert(0) += 1;
                }
            }
        }

        for (key, val) in map {
            if key == val {
                return key as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
