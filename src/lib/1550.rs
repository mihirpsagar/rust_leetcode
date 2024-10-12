// Time taken: 19:11, 19:12 -> Acc
struct Solution {}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for num in arr {
            if num % 2 != 0 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        );
    }
}
