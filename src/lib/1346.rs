use std::collections::HashSet;

// Time taken: 22:05, 22:08 -> Wrong, 22:10 -> Acc
struct Solution {}

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for num in arr {
            if set.contains(&num) {
                return true;
            }
            if num % 2 == 0 {
                set.insert(num / 2);
            }
            set.insert(num * 2);
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
        assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
        assert_eq!(Solution::check_if_exist(vec![7, 1, 14, 11]), true);
    }
}
