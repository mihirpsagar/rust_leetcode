// Time taken: 23:17, 23:21 -> Acc
struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let threshold = (arr.len() / 4) + 1;
        let mut prev_val = arr[0];
        let mut count = 1;
        let mut idx = 1;

        while idx < arr.len() {
            if arr[idx] == prev_val {
                count += 1;
                if count == threshold {
                    return prev_val;
                }
            } else {
                count = 1;
                prev_val = arr[idx];
            }
            idx += 1;
        }

        return prev_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
    }
}
