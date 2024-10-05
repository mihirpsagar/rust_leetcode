// Time taken: 00:55, 01:01 -> Wrong, 01:09 -> Acc
struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::new();
        let mut val = 0;

        for num in nums {
            val = ((val * 2) + num) % 5;
            result.push(val == 0);
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
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            [true, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            [false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ]),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ]
        );
    }
}
