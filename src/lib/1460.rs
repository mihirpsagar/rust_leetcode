use std::collections::HashMap;

// Time taken: 13:28, 13:31 -> Acc
struct Solution {}

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in target {
            *map.entry(num).or_insert(0) += 1;
        }

        for num in arr {
            *map.entry(num).or_insert(0) -= 1;
        }

        for &val in map.values() {
            if val != 0 {
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
        assert_eq!(
            Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
            true
        );
        assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
        assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    }
}
