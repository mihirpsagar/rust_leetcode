// Time taken: 23:44, 23:56 -> Acc
struct Solution {}

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];

        let mut idx = 1;
        while idx < nums.len() {
            if nums[idx] > max {
                max = nums[idx];
            }
            if nums[idx] < min {
                min = nums[idx];
            }
            idx += 1;
        }

        let mut diff = max - min;
        if diff % 2 == 0 {
            diff = diff / 2;
        } else {
            diff = (diff / 2) + 1;
        }

        if diff <= k {
            return 0;
        } else {
            return (max - k) - (min + k);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
