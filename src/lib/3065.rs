// Time taken: 23:54, 23:55 -> Acc
struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for num in nums {
            if num < k {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 3);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 1), 0);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 9), 4);
    }
}
