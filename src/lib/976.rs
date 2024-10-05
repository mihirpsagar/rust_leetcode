// Time taken: 13:46, 13:51 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut idx = nums.len() - 3;
        loop {
            let val1 = nums[idx + 2];
            let val2 = nums[idx + 1];
            let val3 = nums[idx];

            if val1 + val2 > val3 && val2 + val3 > val1 && val3 + val1 > val2 {
                return val1 + val2 + val3;
            }

            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1, 10]), 0);
    }
}
