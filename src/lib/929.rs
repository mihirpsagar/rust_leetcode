use std::collections::HashSet;

// Time taken: 01:25, 01:40 -> Wrong, 01:44 -> Acc
struct Solution {}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for email in emails {
            let email: Vec<char> = email.chars().collect();
            let mut local = Vec::new();
            let mut domain = Vec::new();
            let mut idx = 0;
            let mut is_local = true;

            while idx < email.len() {
                if is_local {
                    if email[idx] == '@' {
                        is_local = false;
                    } else if email[idx] == '+' {
                        is_local = false;
                        while email[idx] != '@' {
                            idx += 1;
                        }
                    } else if email[idx] == '.' {
                        idx += 1;
                        continue;
                    } else {
                        local.push(email[idx]);
                    }
                } else {
                    domain.push(email[idx]);
                }
                idx += 1;
            }

            let valid_email: String =
                local.iter().collect::<String>() + "@" + &domain.iter().collect::<String>();
            set.insert(valid_email);
        }

        return set.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@leetcode.com".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.email@leetcode.com".to_string()
            ]),
            1
        );
    }
}
