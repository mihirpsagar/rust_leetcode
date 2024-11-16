// Time taken: 12:53, 13:08 -> Wrong, 13:14 -> Acc

struct Solution {}

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        return Self::longest_arr(&nums, 0, nums.len(), threshold);
    }

    pub fn longest_arr(arr: &Vec<i32>, start: usize, end: usize, threshold: i32) -> i32 {
        if start >= end {
            return 0;
        }
        let mut idx = start;
        let mut result = 0;
        let mut curr = 0;
        let mut req = 0;

        while idx < end {
            if arr[idx] > threshold {
                result = std::cmp::max(result, curr);
                return std::cmp::max(result, Self::longest_arr(&arr, idx + 1, end, threshold));
            }

            if arr[idx] % 2 == req {
                curr += 1;
                req = (req + 1) % 2;
            } else {
                result = std::cmp::max(result, curr);
                if req == 1 {
                    curr = 1;
                    req = 1;
                } else {
                    curr = 0;
                    req = 0;
                }
            }

            idx += 1;
        }

        result = std::cmp::max(result, curr);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5),
            3
        );
        assert_eq!(Solution::longest_alternating_subarray(vec![1, 2], 2), 1);
        assert_eq!(
            Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4),
            3
        );
        assert_eq!(
            Solution::longest_alternating_subarray(vec![4, 10, 3], 10),
            2
        );
    }
}
