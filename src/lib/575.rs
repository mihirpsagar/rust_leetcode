use std::collections::HashSet;

// Time taken: 22:55, 22:59 -> Acc
struct Solution {}

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let max_len = candy_type.len() / 2;
        for num in candy_type {
            set.insert(num);
            if set.len() >= max_len {
                return max_len as i32;
            }
        }

        return std::cmp::min(max_len as i32, set.len() as i32);
    }

    // pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    //     let mut set = HashSet::new();
    //     let len = candy_type.len();
    //     for num in candy_type {
    //         set.insert(num);
    //     }

    //     return std::cmp::min((len / 2) as i32, set.len() as i32);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
