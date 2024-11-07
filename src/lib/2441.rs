use std::collections::HashSet;

// Time taken: 03:02, 03:04 -> Acc
struct Solution {}

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut result = None;
        let mut set = HashSet::new();

        for num in nums {
            if set.contains(&(num * -1)) {
                if let Some(val) = result {
                    if num.abs() > val {
                        result = Some(num.abs());
                    }
                } else {
                    result = Some(num.abs());
                }
            }
            set.insert(num);
        }

        if let Some(val) = result {
            return val;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}
