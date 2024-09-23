// Time taken: 20:29, 20:31, 20:38 -> Acc
struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut curr = 0;

        for num in nums {
            if num == 1 {
                curr += 1;
            } else {
                if curr > result {
                    result = curr;
                }
                curr = 0;
            }
        }

        if curr > result {
            result = curr;
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
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
