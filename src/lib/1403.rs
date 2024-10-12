// Time taken: 10:33, 10:38 -> Acc
struct Solution {}

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        nums.sort();

        let mut idx = nums.len() - 1;
        let mut arr_sum: i32 = nums.iter().sum();
        let mut result_sum = 0;

        loop {
            result.push(nums[idx]);
            arr_sum -= nums[idx];
            result_sum += nums[idx];
            if result_sum > arr_sum {
                break;
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), [10, 9]);
        assert_eq!(Solution::min_subsequence(vec![4, 4, 7, 6, 7]), [7, 7, 6]);
    }
}
