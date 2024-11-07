// Time taken: 23:43, 23:45 -> Acc
struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;

        while left < nums1.len() && right < nums2.len() {
            if nums1[left] == nums2[right] {
                return nums1[left];
            } else if nums1[left] < nums2[right] {
                left += 1;
            } else {
                right += 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
