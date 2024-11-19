// Time taken: 11:25, 11:27 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < nums1.len() {
            let mut idx2 = 0;
            while idx2 < nums2.len() {
                if nums1[idx1] % (nums2[idx2] * k) == 0 {
                    result += 1;
                }

                idx2 += 1;
            }

            idx1 += 1;
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
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1),
            5
        );
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3),
            2
        );
    }
}
