// Time taken: 17:04, 17:10 -> Acc
struct Solution {}

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut result = 0;

        for i in 0..len {
            for j in i..len {
                if Self::is_valid(&nums, i, j) {
                    result += 1;
                }
            }
        }

        return result;
    }

    pub fn is_valid(arr: &Vec<i32>, left: usize, right: usize) -> bool {
        let mut prev = i32::MIN;
        let mut idx = 0;

        while idx < arr.len() {
            if idx >= left && idx <= right {
                idx += 1;
                continue;
            }

            if prev >= arr[idx] {
                return false;
            }

            prev = arr[idx];
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
        assert_eq!(Solution::incremovable_subarray_count(vec![8, 7, 6, 6]), 3);
    }
}
