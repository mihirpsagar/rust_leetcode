// Time taken: 12:32, 12:34 -> Acc
struct Solution {}

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut single_sum = 0;
        let mut double_sum = 0;

        for num in nums {
            if num < 10 {
                single_sum += num;
            } else {
                double_sum += num;
            }
        }

        return single_sum != double_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 10]), false);
        assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]), true);
        assert_eq!(Solution::can_alice_win(vec![5, 5, 5, 25]), true);
    }
}
