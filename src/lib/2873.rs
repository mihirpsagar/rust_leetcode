// Time taken: 10:57, 11:01 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let len = nums.len();
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < len - 2 {
            let mut idx2 = idx1 + 1;

            while idx2 < len - 1 {
                let diff = (nums[idx1] - nums[idx2]) as i64;
                let mut idx3 = idx2 + 1;

                while idx3 < len {
                    result = std::cmp::max(result, diff * nums[idx3] as i64);

                    idx3 += 1;
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
        assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
