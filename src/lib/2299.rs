// Time taken: 13:00, 13:10 -> Acc
struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let password = password.chars().collect::<Vec<char>>();
        let special_char_arr = "!@#$%^&*()-+";
        let mut has_lower_case = false;
        let mut has_upper_case = false;
        let mut has_digit = false;
        let mut has_special_char = false;
        let mut prev = None;

        if password.len() < 8 {
            return false;
        }

        let mut idx = 0;
        while idx < password.len() {
            if password[idx].is_ascii_alphabetic() {
                if password[idx].is_ascii_lowercase() {
                    has_lower_case = true;
                } else {
                    has_upper_case = true;
                }
            } else if password[idx].is_ascii_digit() {
                has_digit = true;
            } else if special_char_arr.contains(password[idx]) {
                has_special_char = true;
            } else {
                return false;
            }

            if let Some(prev_ch) = prev {
                if prev_ch == password[idx] {
                    return false;
                }
            }
            prev = Some(password[idx]);
            idx += 1;
        }

        if !has_lower_case || !has_upper_case || !has_digit || !has_special_char {
            return false;
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
            Solution::strong_password_checker_ii("IloveLe3tcode!".to_string()),
            true
        );
        assert_eq!(
            Solution::strong_password_checker_ii("Me+You--IsMyDream".to_string()),
            false
        );
        assert_eq!(
            Solution::strong_password_checker_ii("1aB!".to_string()),
            false
        );
    }
}
