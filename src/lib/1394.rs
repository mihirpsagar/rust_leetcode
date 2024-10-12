use std::collections::HashMap;

// Time taken: 10:16, 10:18 -> Acc
struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result = (-1, -1);
        for (key, val) in map {
            if val > result.1 && key == val {
                result = (key, val);
            }
        }

        return result.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    }
}
