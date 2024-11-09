// Time taken: 06:57, 06:59 -> Acc
struct Solution {}

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for num in nums {
            let mut arr = Vec::new();
            let mut val = num;
            while val > 0 {
                arr.push(val % 10);
                val /= 10;
            }
            result.extend(arr.iter().rev());
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
            Solution::separate_digits(vec![13, 25, 83, 77]),
            [1, 3, 2, 5, 8, 3, 7, 7]
        );
        assert_eq!(Solution::separate_digits(vec![7, 1, 3, 9]), [7, 1, 3, 9]);
    }
}
