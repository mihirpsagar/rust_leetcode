// Time taken: 13:42, 13:45 -> Acc
struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut idx1 = 0;
        while idx1 < prices.len() {
            let mut idx2 = idx1 + 1;
            let mut discount = 0;
            while idx2 < prices.len() {
                if prices[idx2] <= prices[idx1] {
                    discount = prices[idx2];
                    break;
                }
                idx2 += 1;
            }
            result.push(prices[idx1] - discount);
            idx1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::final_prices(vec![8, 4, 6, 2, 3]), [4, 2, 4, 2, 3]);
        assert_eq!(Solution::final_prices(vec![1, 2, 3, 4, 5]), [1, 2, 3, 4, 5]);
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), [9, 0, 1, 6]);
    }
}
