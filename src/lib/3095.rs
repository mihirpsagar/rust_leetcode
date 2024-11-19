// Time taken: 10:20, 10:28 -> Wrong, 10:30 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = i32::MAX;
        let mut found = false;

        for num in nums.iter() {
            if *num >= k {
                return 1;
            }
        }

        let mut left = 0;
        while left < nums.len() {
            let mut right = left;
            while right < nums.len() {
                let val = Self::get_or(&nums, left, right);
                if val >= k {
                    found = true;
                    result = std::cmp::min(result, (right - left + 1) as i32);
                }

                right += 1;
            }

            left += 1;
        }

        if !found {
            return -1;
        } else {
            return result;
        }
    }

    pub fn get_or(arr: &Vec<i32>, left: usize, right: usize) -> i32 {
        let mut result = arr[left];
        let mut idx = left + 1;
        while idx <= right {
            result = result | arr[idx];
            idx += 1;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
        assert_eq!(
            Solution::minimum_subarray_length(vec![32, 1, 25, 11, 2], 59),
            4
        );
    }
}
