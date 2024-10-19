use std::collections::HashSet;

// Time taken: 00:00, 00:10 -> Wrong, 00:21 -> Acc
struct Solution {}

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set = HashSet::new();
        let word = word.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut number = String::new();

        while idx < word.len() {
            if !word[idx].is_ascii_digit() {
                if !number.is_empty() {
                    let filtered_number = Self::filter_number(number.clone());
                    set.insert(filtered_number.clone());
                    number.clear();
                }
            } else {
                number.push(word[idx]);
            }
            idx += 1;
        }

        if !number.is_empty() {
            let filtered_number = Self::filter_number(number.clone());
            set.insert(filtered_number.clone());
        }

        return set.len() as i32;
    }

    pub fn filter_number(number: String) -> String {
        let mut filtered_number = String::new();
        let mut leading_zero = true;
        for ch in number.chars() {
            if ch != '0' {
                filtered_number.push(ch);
                leading_zero = false;
            } else {
                if !leading_zero {
                    filtered_number.push(ch);
                }
            }
        }
        if filtered_number.is_empty() {
            filtered_number.push('0');
        }

        return filtered_number;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_different_integers("a123bc34d8ef34".to_string()),
            3
        );
        assert_eq!(
            Solution::num_different_integers("leet1234code234".to_string()),
            2
        );
        assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
        assert_eq!(Solution::num_different_integers("0a0".to_string()), 1);
    }
}
