// Time taken: 19:21, 19:25 -> Acc
struct Solution {}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut sum = 0;
        let mut idx = 0;
        while idx < nums.len() {
            sum += std::cmp::min(nums[idx], nums[idx + 1]);
            idx += 2;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
