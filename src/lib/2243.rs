// Time taken: 01:33, 01:51 -> Wrong, 01:54 -> Acc
struct Solution {}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut arr = Vec::new();
        let mut result = Vec::new();
        let k = k as usize;
        let zero_ascii = '0' as u8;

        for ch in s.chars() {
            arr.push((ch as u8 - zero_ascii) as u32);
        }

        while arr.len() > k {
            let mut arr2 = Vec::new();

            let mut idx = 0;
            while idx < arr.len() {
                let mut sum_arr = Self::sum_arr(&arr, idx, k);
                arr2.append(&mut sum_arr);
                idx += k;
            }

            arr = arr2;
        }

        for val in arr {
            result.push((val as u8 + zero_ascii) as char);
        }

        return result.iter().collect();
    }

    pub fn sum_arr(arr: &Vec<u32>, idx: usize, k: usize) -> Vec<u32> {
        let mut result = Vec::new();
        let threshold = idx + k;
        let mut idx = idx;
        let mut sum = 0;

        while idx < arr.len() && idx < threshold {
            sum += arr[idx];
            idx += 1;
        }

        while sum > 0 {
            result.push(sum % 10);
            sum /= 10;
        }

        result.reverse();

        if result.is_empty() {
            result.push(0);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::digit_sum("11111222223".to_string(), 3), "135");
        assert_eq!(Solution::digit_sum("00000000".to_string(), 3), "000");
        assert_eq!(Solution::digit_sum("71818186138735364590516322993378229838446988388364431324753408563431136824898916288399".to_string(), 85), "4169");
    }
}
