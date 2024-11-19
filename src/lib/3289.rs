use std::collections::HashSet;

// Time taken: 15:21, 15:23 -> Acc
struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut set = HashSet::new();

        for num in nums {
            if !set.insert(num) {
                result.push(num);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_sneaky_numbers(vec![0, 1, 1, 0]), [1, 0]);
        assert_eq!(Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]), [3, 2]);
        assert_eq!(
            Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
            [4, 5]
        );
    }
}
