// Time taken: 15:45, 15:55 -> Acc
struct Solution {}

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_count = Vec::new();
        for num in nums {
            let binary = Self::get_binary(num);
            Self::add_binary_count(&mut sum_count, &binary);
        }

        sum_count.reverse();

        let mut result = 0;
        for bit in sum_count {
            if bit >= k {
                result = (result * 2) + 1;
            } else {
                result *= 2;
            }
        }

        return result;
    }

    pub fn get_binary(mut num: i32) -> Vec<i32> {
        let mut result = Vec::new();
        while num > 0 {
            result.push(num % 2);
            num /= 2;
        }

        return result;
    }

    pub fn add_binary_count(arr: &mut Vec<i32>, b_arr: &Vec<i32>) {
        let mut idx = 0;
        while idx < b_arr.len() {
            if let Some(val) = arr.get_mut(idx) {
                *val += b_arr[idx];
            } else {
                arr.push(b_arr[idx]);
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
