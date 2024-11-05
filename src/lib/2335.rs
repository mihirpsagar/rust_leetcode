// Time taken: 21:33, 21:36, 21:44 -> Acc
struct Solution {}

impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        let mut result = 0;
        amount.sort();

        loop {
            if amount[0] == 0 {
                break;
            }
            amount[0] -= 1;
            if amount[2] > amount[1] {
                amount[2] -= 1;
            } else {
                amount[1] -= 1;
            }
            result += 1;
        }

        result += amount[1];
        result += amount[2] - amount[1];

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }
}
