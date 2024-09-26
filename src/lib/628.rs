// Time taken: 23:23, 23:50 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        if len == 3 {
            return nums[0] * nums[1] * nums[2];
        }

        nums.sort();

        let first_two_neg = nums[0] * nums[1] * nums[len - 1];
        let last_two_pos = nums[len - 2] * nums[len - 3] * nums[len - 1];

        return std::cmp::max(first_two_neg, last_two_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
