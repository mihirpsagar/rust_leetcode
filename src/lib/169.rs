// Time taken: 12:35, 12:41 -> Acc
struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                result = num;
                count += 1;
            } else {
                if result == num {
                    count += 1;
                } else {
                    count -= 1;
                }
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
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
