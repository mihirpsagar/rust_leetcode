// Time taken: 11:37, 11:41 -> Acc
struct Solution {}

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut double = 0;

        for num in player1 {
            if double > 0 {
                sum1 = sum1 + (2 * num);
                double -= 1;
            } else {
                sum1 += num;
            }

            if num == 10 {
                double = 2;
            }
        }

        double = 0;
        for num in player2 {
            if double > 0 {
                sum2 = sum2 + (2 * num);
                double -= 1;
            } else {
                sum2 += num;
            }

            if num == 10 {
                double = 2;
            }
        }

        if sum1 > sum2 {
            return 1;
        } else if sum1 == sum2 {
            return 0;
        } else {
            return 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_winner(vec![5, 10, 3, 2], vec![6, 5, 7, 3]), 1);
        assert_eq!(Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
        assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0);
        assert_eq!(
            Solution::is_winner(vec![1, 1, 1, 10, 10, 10, 10], vec![10, 10, 10, 10, 1, 1, 1]),
            2
        );
    }
}
