// Time taken: 12:44, 12:54 -> Acc
struct Solution {}

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();
        let count = (arr.len() as f64 * 0.05) as usize;
        let mut result = 0;
        let mut idx = 0;
        let mut len = 0;

        while idx < arr.len() {
            if idx >= count && idx < arr.len() - count {
                result += arr[idx];
                len += 1;
            }
            idx += 1;
        }

        if len != 0 {
            return result as f64 / len as f64;
        }

        return 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.0
        );
        assert_eq!(
            Solution::trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ]),
            4.0
        );
        assert_eq!(
            Solution::trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
            ]),
            4.777777777777778
        );
    }
}
