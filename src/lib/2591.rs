// Time taken: 10:23, 10:29, 10:32 -> Wrong, 10:35 -> Acc
struct Solution {}

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }

        let mut money = money - children;
        let mut result = std::cmp::min(money / 7, children - 1);
        money = money - (result * 7);
        if money == 3 && result == children - 1 {
            result = std::cmp::max(0, result - 1);
        } else if money == 7 {
            result += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::dist_money(20, 3), 1);
        assert_eq!(Solution::dist_money(16, 2), 2);
        assert_eq!(Solution::dist_money(1, 2), -1);
    }
}
