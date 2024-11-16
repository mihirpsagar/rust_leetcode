// Time taken: 13:48, 13:52 -> Acc
struct Solution {}

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut min = (i32::MAX, i32::MAX);

        for price in prices {
            if price < min.0 {
                min.1 = min.0;
                min.0 = price;
            } else if price < min.1 {
                min.1 = price;
            }
        }

        let balance = money - min.0 - min.1;
        if balance >= 0 {
            return balance;
        } else {
            return money;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
