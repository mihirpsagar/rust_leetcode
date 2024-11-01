use std::collections::HashSet;

// Time taken: 21:56, 21:58 -> Acc
struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut set = HashSet::new();
        let mut result = original;
        for num in nums {
            set.insert(num);
        }

        loop {
            if set.contains(&result) {
                result *= 2;
            } else {
                break;
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
        assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
        assert_eq!(Solution::find_final_value(vec![2, 7, 9], 4), 4);
    }
}
