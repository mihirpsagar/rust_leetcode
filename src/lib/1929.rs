// Time taken: 20:27, 20:30 -> Acc
struct Solution {}

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let mut idx = 0;
        let len = nums.len();
        while idx < len {
            nums.push(nums[idx]);
            idx += 1;
        }
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            [1, 2, 1, 1, 2, 1]
        );
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            [1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
