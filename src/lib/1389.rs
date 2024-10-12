// Time taken: 10:12, 10:14 -> Acc
struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            result.insert(index[idx] as usize, nums[idx]);
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
        assert_eq!(
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            [0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), [1]);
    }
}
