// Time taken: 21:21, 21:23 -> Acc

struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase(mut purchase_amount: i32) -> i32 {
        let last_digit = purchase_amount % 10;
        if last_digit >= 5 {
            purchase_amount = purchase_amount + (10 - last_digit);
        } else {
            purchase_amount -= last_digit;
        }

        return 100 - purchase_amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
        assert_eq!(Solution::account_balance_after_purchase(10), 90);
    }
}
