// Time taken: 22:47, 22:49, 23:01
struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = (0, 0);
        for num in bills {
            if num == 5 {
                change.0 += 1;
            } else if num == 10 {
                change.0 -= 1;
                if change.0 < 0 {
                    return false;
                }
                change.1 += 1;
            } else {
                if change.1 > 0 && change.0 > 0 {
                    change.1 -= 1;
                    change.0 -= 1;
                } else if change.0 >= 3 {
                    change.0 -= 3;
                } else {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}
