// Time taken: 20:38, 20:40 -> Acc
struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut neg_count = 0;

        for num in nums {
            if num < 0 {
                neg_count += 1;
            } else {
                if num == 0 {
                    return 0;
                }
            }
        }

        if neg_count % 2 == 0 {
            return 1;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
