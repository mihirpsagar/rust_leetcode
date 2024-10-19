// Time taken: 11:35, 11:52 -> Acc
struct Solution {}

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut drop = 0;
        let mut idx = 0;
        let len = nums.len();

        while idx < len {
            if nums[idx] > nums[(idx + 1) % len] {
                drop += 1;
                if drop == 2 {
                    return false;
                }
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check(vec![3, 4, 5, 1, 2]), true);
        assert_eq!(Solution::check(vec![2, 1, 3, 4]), false);
        assert_eq!(Solution::check(vec![1, 2, 3]), true);
    }
}
