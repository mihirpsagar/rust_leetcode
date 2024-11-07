// Time taken: 03:28, 03:30 -> Acc
struct Solution {}

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut sum = 0;

        for num in nums {
            if num % 3 == 0 {
                total += 1;
                sum += num;
            }
        }

        if total == 0 {
            return 0;
        }

        return sum / total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
        assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
    }
}
