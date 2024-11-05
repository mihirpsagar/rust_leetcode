use std::collections::HashMap;

// Time taken: 21:47, 21:50 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = vec![0, 0];

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for &val in map.values() {
            result[0] = result[0] + (val / 2);
            result[1] = result[1] + (val % 2);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]), [3, 1]);
        assert_eq!(Solution::number_of_pairs(vec![1, 1]), [1, 0]);
        assert_eq!(Solution::number_of_pairs(vec![0]), [0, 1]);
    }
}
