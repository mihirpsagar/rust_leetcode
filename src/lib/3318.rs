use std::collections::HashMap;

// Time taken: 15:49, 15:58 -> Wrong, 16:08 -> Wrong, 16:10 -> Acc
struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = 0;
        let len = nums.len();
        let k = k as usize;

        while idx + k - 1 < len {
            result.push(Self::get_freq_sum(&nums, idx, idx + k - 1, x));
            idx += 1;
        }

        return result;
    }

    pub fn get_freq_sum(arr: &Vec<i32>, start: usize, end: usize, x: i32) -> i32 {
        let mut map = HashMap::new();
        let mut idx = start;
        while idx <= end {
            *map.entry(arr[idx]).or_insert(0) += 1;
            idx += 1;
        }

        let mut arr = Vec::new();
        for (key, val) in map {
            arr.push((key, val));
        }

        arr.sort_by(|a, b| {
            if a.1 != b.1 {
                return b.1.cmp(&a.1);
            } else {
                return b.0.cmp(&a.0);
            }
        });

        let mut sum = 0;
        let mut idx = 0;
        let mut x = x;
        while idx < arr.len() && x > 0 {
            sum = sum + (arr[idx].0 * arr[idx].1);
            idx += 1;
            x -= 1;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            [6, 10, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            [11, 15, 15, 15, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![50, 50, 50, 50, 50, 50], 6, 1),
            [300]
        );
        assert_eq!(Solution::find_x_sum(vec![1, 2, 3, 4, 5, 6], 6, 1), [6]);
        assert_eq!(Solution::find_x_sum(vec![9, 2, 2], 3, 3), [13]);
    }
}
