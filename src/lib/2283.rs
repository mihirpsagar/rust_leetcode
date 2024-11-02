// Time taken: 02:52, 02:57 -> Acc
struct Solution {}

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count: Vec<usize> = vec![0; 10];
        let num = num.chars().collect::<Vec<char>>();
        let zero_ascii = '0' as u8;
        let mut idx = 0;

        while idx < num.len() {
            count[(num[idx] as u8 - zero_ascii) as usize] += 1;
            idx += 1;
        }

        idx = 0;
        while idx < num.len() {
            if (num[idx] as u8 - zero_ascii) as usize != count[idx] {
                return false;
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::digit_count("1210".to_string()), true);
        assert_eq!(Solution::digit_count("030".to_string()), false);
    }
}
