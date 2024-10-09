use std::collections::{HashMap, HashSet};

// Time taken: 23:43, 23:45 -> Acc
struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let mut set = HashSet::new();

        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }

        for val in map.values() {
            if !set.insert(val) {
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
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
