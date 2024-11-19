// Time taken: 11:00, 11:04 -> Acc
struct Solution {}

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min1 = nums1[0];
        let mut min2 = nums2[0];
        for num in nums1 {
            min1 = std::cmp::min(min1, num);
        }
        for num in nums2 {
            min2 = std::cmp::min(min2, num);
        }

        return min2 - min1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
        assert_eq!(Solution::added_integer(vec![10], vec![5]), -5);
        assert_eq!(
            Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]),
            0
        );
    }
}
