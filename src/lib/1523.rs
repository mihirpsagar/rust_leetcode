// Time taken: 17:46, 17:56 -> Acc
struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let val = high - low + 1;

        if val % 2 == 0 {
            return val / 2;
        } else {
            if low % 2 == 0 {
                return val / 2;
            } else {
                return (val / 2) + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_odds(3, 7), 3);
        assert_eq!(Solution::count_odds(8, 10), 1);
    }
}
