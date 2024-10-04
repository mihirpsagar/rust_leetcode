// Time taken: 00:54, 00:58, 01:02 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 1;
        let mut idx = 0;
        let mut nums2 = vec![0; nums.len()];

        while idx < nums.len() {
            if nums[idx] % 2 == 0 {
                nums2[even] = nums[idx];
                even += 2;
            } else {
                nums2[odd] = nums[idx];
                odd += 2;
            }
            idx += 1;
        }

        return nums2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            [4, 5, 2, 7]
        );
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), [2, 3]);
    }
}
