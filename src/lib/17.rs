// Time taken: 21:24, 21:39 -> Acc
struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut arr = Vec::new();
        let mut letter = 'a';
        let zero = '0' as u8;
        let digits = digits.chars().collect::<Vec<char>>();

        for k in 0..8 {
            let mut row = Vec::new();
            if k == 5 || k == 7 {
                for _ in 0..4 {
                    row.push(letter);
                    letter = (letter as u8 + 1) as char;
                }
            } else {
                for _ in 0..3 {
                    row.push(letter);
                    letter = (letter as u8 + 1) as char;
                }
            }
            arr.push(row);
        }

        for val in &arr[(digits[0] as u8 - zero - 2) as usize] {
            result.push(vec![*val]);
        }

        let mut idx = 1;
        while idx < digits.len() {
            let chars = &arr[(digits[idx] as u8 - zero - 2) as usize];
            let mut new_result = Vec::new();
            for val in chars {
                let mut new_rows = result.clone();
                for row in new_rows.iter_mut() {
                    row.push(*val);
                }
                new_result.append(&mut new_rows);
            }
            result = new_result;
            idx += 1;
        }

        let mut parsed_result = Vec::new();
        for str in result {
            parsed_result.push(str.iter().collect());
        }

        return parsed_result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()).is_empty(),
            true
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            ["a", "b", "c"]
        );
    }
}
