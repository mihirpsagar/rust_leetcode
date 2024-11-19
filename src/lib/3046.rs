use std::collections::HashMap;

// Time taken: 23:51, 23:52 -> Acc
struct Solution {}

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for val in map.values() {
            if *val > 2 {
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
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]), true);
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 1, 1]), false);
    }
}
