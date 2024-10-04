// Time taken: 23:40, 23:42 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut idx = 0;

        while idx < nums.len() {
            if nums[idx] % 2 == 0 {
                let tmp = nums[idx];
                nums[idx] = nums[left];
                nums[left] = tmp;
                left += 1;
            }
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
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            [2, 4, 3, 1]
        );
        assert_eq!(Solution::sort_array_by_parity(vec![0]), [0]);
    }
}
