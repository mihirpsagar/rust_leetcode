use std::collections::HashMap;

// Time taken: 13:43, 13:50 -> Acc
struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;

        for &num in nums.iter() {
            if let Some(val) = map.get(&(num - k)) {
                result += val;
            }
            if let Some(val) = map.get(&(num + k)) {
                result += val;
            }

            *map.entry(num).or_insert(0) += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
