// Time taken: 13:16, 13:29 -> Wrong, 13:30 -> Acc

struct Solution {}

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut result: i32 = -1;
        let mut idx1 = 0;
        while idx1 < nums.len() - 1 {
            if nums[idx1 + 1] - nums[idx1] != 1 {
                idx1 += 1;
                continue;
            }

            let mut curr = 2;
            let mut idx2 = idx1 + 2;
            let mut res = -1;
            'inner: while idx2 < nums.len() {
                if nums[idx2] - nums[idx2 - 1] == res {
                    curr += 1;
                    res *= -1;
                    idx2 += 1;
                } else {
                    break 'inner;
                }
            }

            result = std::cmp::max(result, curr);

            idx1 += 1;

            if result >= (nums.len() - idx1) as i32 {
                break;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
        assert_eq!(Solution::alternating_subarray(vec![21, 9, 5]), -1);
    }
}
