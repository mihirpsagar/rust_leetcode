// Time taken: 11:31, 11:44 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        // k = (i + 1) * (n - i)
        // Odd = (k + 1) / 2
        let n = arr.len();
        let mut result = 0;

        for (idx, &num) in arr.iter().enumerate() {
            let k = (idx + 1) * (n - idx);
            result = result + (((k as i32 + 1) / 2) * num);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10]), 10);
    }
}
