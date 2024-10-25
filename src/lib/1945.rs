// Time taken: 21:13, 21:23 -> Acc
struct Solution {}

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum: i32 = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut arr = Vec::new();
        let a_ascii = 'a' as u8;

        for ch in s {
            arr.push((ch as u8 - a_ascii + 1) as i32);
        }

        for _ in 0..k {
            sum = 0;
            while let Some(val) = arr.pop() {
                let mut num = val as i32;
                while num > 0 {
                    sum += num % 10;
                    num /= 10;
                }
            }
            if sum < 10 {
                break;
            }
            arr.push(sum);
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
        assert_eq!(Solution::get_lucky("zbax".to_string(), 2), 8);
    }
}
