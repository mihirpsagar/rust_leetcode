use std::collections::HashSet;

// Time taken: 22:18, 22:21, 22:27 -> Acc
struct Solution {}

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut result = 0;
        let mut set = HashSet::new();

        for num in nums {
            if set.contains(&(num - diff)) && set.contains(&(num - diff - diff)) {
                result += 1;
            }
            set.insert(num);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
