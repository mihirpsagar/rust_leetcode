// Time taken: 12:14, 12:17 -> Acc
struct Solution {}

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let zero_ascii = '0' as u8;
        let mut idx = 0;

        while idx < s.len() - 1 {
            let d1 = s[idx] as u8 - zero_ascii;
            let d2 = s[idx + 1] as u8 - zero_ascii;

            if (d1 % 2 == d2 % 2) && d1 > d2 {
                (s[idx], s[idx + 1]) = (s[idx + 1], s[idx]);
                break;
            }

            idx += 1;
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_smallest_string("45320".to_string()), "43520");
        assert_eq!(Solution::get_smallest_string("001".to_string()), "001");
    }
}
