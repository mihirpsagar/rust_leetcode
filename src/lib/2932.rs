// Time taken: 16:16, 16:20 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut idx1 = 0;
        while idx1 < nums.len() - 1 {
            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if (nums[idx1] - nums[idx2]).abs() <= std::cmp::min(nums[idx1], nums[idx2]) {
                    result = std::cmp::max(result, nums[idx1] ^ nums[idx2]);
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
    }
}
