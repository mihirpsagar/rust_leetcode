// Time taken: 22:50, 22:57 -> Acc
struct Solution {}

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let s = s.chars().collect::<Vec<char>>();
        let start_ch = s[0];
        let end_ch = s[3];
        let start_val = s[1];
        let end_val = s[4];

        let mut ch = start_ch;
        let mut val = start_val;
        loop {
            let mut str = String::from(ch);
            str.push(val);
            result.push(str);

            if val == end_val {
                if ch == end_ch {
                    break;
                }
                ch = (ch as u8 + 1) as char;
                val = start_val;
            } else {
                val = (val as u8 + 1) as char;
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
        assert_eq!(
            Solution::cells_in_range("K1:L2".to_string()),
            ["K1", "K2", "L1", "L2"]
        );
        assert_eq!(
            Solution::cells_in_range("A1:F1".to_string()),
            ["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }
}
