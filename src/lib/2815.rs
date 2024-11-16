// Time taken: 22:50, 22:56 -> Acc

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result = -1;
        for num in nums {
            let max_digit = Self::max_digit(num);
            if let Some(val) = map.get_mut(&max_digit) {
                val.push(num);
            } else {
                map.insert(max_digit, vec![num]);
            }
        }

        for arr in map.values_mut() {
            let len = arr.len();
            if len > 1 {
                arr.sort();
                result = std::cmp::max(result, arr[len - 1] + arr[len - 2]);
            }
        }

        return result;
    }

    pub fn max_digit(num: i32) -> i32 {
        let mut result = 0;
        let mut num = num;

        while num > 0 {
            result = std::cmp::max(result, num % 10);
            num /= 10;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_sum(vec![112, 131, 411]), -1);
        assert_eq!(Solution::max_sum(vec![2536, 1613, 3366, 162]), 5902);
        assert_eq!(Solution::max_sum(vec![51, 71, 17, 24, 42]), 88);
    }
}
