// Time taken: 16:49, 17:04, 17:09 -> Acc
struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        let mut result = vec![0; len];
        let k_abs = k.abs() as usize;

        if k == 0 {
            return result;
        } else if k > 0 {
            let mut sum = 0;
            let mut idx = 1;
            for _ in 0..k_abs {
                sum += code[idx];
                idx = (idx + 1) % len;
            }

            idx = 0;
            while idx < len {
                result[idx] = sum;
                sum -= code[(idx + 1) % len];
                sum += code[(idx + k_abs + 1) % len];
                idx += 1;
            }
        } else {
            let mut sum = 0;
            let mut idx = len - 1;
            for _ in 0..k_abs {
                sum += code[idx];
                idx -= 1;
            }

            idx = 0;
            while idx < len {
                result[idx] = sum;
                if idx < k_abs {
                    sum -= code[len - k_abs + idx];
                } else {
                    sum -= code[idx - k_abs];
                }
                sum += code[idx];
                idx += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), [12, 10, 16, 13]);
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), [0, 0, 0, 0]);
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), [12, 5, 6, 13]);
    }
}
