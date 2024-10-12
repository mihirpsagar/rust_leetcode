// Time taken: 14:29, 14:30 -> Acc
struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < nums.len() {
            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if nums[idx1] == nums[idx2] {
                    result += 1;
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
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
