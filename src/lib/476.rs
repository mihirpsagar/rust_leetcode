// Time taken: 19:41, 19:42, 19:52 -> Acc
struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let b_num = format!("{:0b}", num);
        let mut result = 0;

        for (idx, ch) in b_num.chars().enumerate() {
            let pos = b_num.len() - idx - 1;
            if ch == '0' {
                if pos == 31 {
                    result -= 2_i32.pow(31);
                } else {
                    result += 2_i32.pow(pos as u32);
                }
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
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(1), 0);
    }
}
