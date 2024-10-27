// Time taken: 20:06, 20:09 -> Acc
struct Solution {}

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (idx, &num) in nums.iter().enumerate() {
            if idx as i32 % 10 == num {
                return idx as i32;
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
        assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
        assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
        assert_eq!(
            Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            -1
        );
    }
}
