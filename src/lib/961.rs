use std::collections::HashSet;

// Time taken: 13:34, 13:36 -> Acc
struct Solution {}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for num in nums {
            if !set.insert(num) {
                return num;
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
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
