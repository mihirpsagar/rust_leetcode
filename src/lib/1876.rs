// Time taken: 18:32, 18:38 -> Acc
struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut arr = vec![0; 26];
        let a_ascii = 'a' as u8;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 2;

        if s.len() < 3 {
            return 0;
        }

        arr[(s[0] as u8 - a_ascii) as usize] += 1;
        arr[(s[1] as u8 - a_ascii) as usize] += 1;

        while idx < s.len() {
            arr[(s[idx] as u8 - a_ascii) as usize] += 1;

            if Self::is_valid(&arr) {
                result += 1;
            }

            arr[(s[idx - 2] as u8 - a_ascii) as usize] -= 1;
            idx += 1;
        }

        return result;
    }

    pub fn is_valid(arr: &Vec<i32>) -> bool {
        for &num in arr {
            if num > 1 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
        assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    }
}
