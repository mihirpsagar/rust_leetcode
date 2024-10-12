// Time taken: 13:33, 13:37 -> Acc
struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = (nums[0], nums[1]);

        if max.0 < max.1 {
            let tmp = max.0;
            max.0 = max.1;
            max.1 = tmp;
        }

        let mut idx = 2;
        while idx < nums.len() {
            if nums[idx] > max.0 {
                max.1 = max.0;
                max.0 = nums[idx];
            } else if nums[idx] > max.1 {
                max.1 = nums[idx];
            }
            idx += 1;
        }

        return (max.0 - 1) * (max.1 - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
