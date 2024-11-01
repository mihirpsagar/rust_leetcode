use std::collections::HashMap;

// Time taken: 22:42, 22:47, 22:49 -> Acc
struct Solution {}

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = HashMap::new();
        let mut idx = 0;
        while idx < nums.len() - 1 {
            if nums[idx] == key {
                *map.entry(nums[idx + 1]).or_insert(0) += 1;
            }
            idx += 1;
        }

        let mut max: Option<(i32, i32)> = None;

        for (key, val) in map {
            if let Some(prev) = max {
                if prev.1 < val {
                    max = Some((key, val));
                }
            } else {
                max = Some((key, val));
            }
        }

        return max.unwrap().0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
        assert_eq!(Solution::most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
    }
}
