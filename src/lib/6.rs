// Time taken: 13:05, 13:14, 13:25 -> Acc
struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut converted_str = Vec::new();
        let mut row = 0;
        let mut increment = true;
        for (idx, ch) in s.chars().enumerate() {
            converted_str.push((ch, row, idx));

            if increment {
                row += 1;
                if row == num_rows {
                    increment = false;
                    row = num_rows - 2;
                }
            } else {
                row -= 1;
                if row == -1 {
                    increment = true;
                    row = 1;
                }
            }
        }

        converted_str.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.2.cmp(&b.2);
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut result = Vec::new();
        for ch in converted_str {
            result.push(ch.0);
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }
}
