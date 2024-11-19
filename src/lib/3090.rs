// Time taken: 10:10, 10:18 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut count = vec![0; 26];
        let a_ascii = 'a' as u8;
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = 2;
        let mut result = 2;

        count[(s[0] as u8 - a_ascii) as usize] += 1;
        count[(s[1] as u8 - a_ascii) as usize] += 1;

        while right < s.len() {
            count[(s[right] as u8 - a_ascii) as usize] += 1;

            if Self::is_valid(&count) {
                result = std::cmp::max(result, right - left + 1);
            } else {
                while !Self::is_valid(&count) {
                    count[(s[left] as u8 - a_ascii) as usize] -= 1;
                    left += 1;
                }
            }

            right += 1;
        }

        return result as i32;
    }

    pub fn is_valid(arr: &Vec<i32>) -> bool {
        for num in arr {
            if *num > 2 {
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
        assert_eq!(
            Solution::maximum_length_substring("bcbbbcba".to_string()),
            4
        );
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }
}
