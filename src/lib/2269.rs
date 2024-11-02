// Time taken: 02:31, 02:37 -> Acc
struct Solution {}

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let mut arr = Vec::new();
        let mut val = num;

        while val > 0 {
            arr.push(val % 10);
            val /= 10;
        }

        arr.reverse();

        let mut idx = 0;
        while idx <= arr.len() - k {
            if Self::is_divisor(&arr, idx, num, k) {
                result += 1;
            }
            idx += 1;
        }

        return result;
    }

    pub fn is_divisor(arr: &Vec<i32>, idx: usize, num: i32, k: usize) -> bool {
        let mut val = 0;
        let threshold = idx + k;
        let mut idx = idx;

        while idx < threshold {
            val *= 10;
            val += arr[idx];
            idx += 1;
        }

        if val == 0 {
            return false;
        }

        return num % val == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    }
}
