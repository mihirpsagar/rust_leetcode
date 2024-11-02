// Time taken: 01:29, 01:32 -> Acc
struct Solution {}

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];

        for num in nums {
            if num.abs() < result.abs() {
                result = num;
            } else if num.abs() == result.abs() && num > result {
                result = num;
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
        assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
        assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
    }
}
