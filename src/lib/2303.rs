// Time taken: 13:12, 13:19 -> Acc
struct Solution {}

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut result = 0.0;
        let mut prev = 0;
        let mut remaining = income;

        for bracket in brackets {
            if remaining <= 0 {
                break;
            }
            let curr = std::cmp::min(remaining, bracket[0] - prev);
            result = result + ((curr * bracket[1]) as f64 / 100.00);
            prev = bracket[0];
            remaining -= curr;
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
            Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65000
        );
        assert_eq!(
            Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25000
        );
        assert_eq!(Solution::calculate_tax(vec![vec![2, 50]], 0), 0.00000);
    }
}
