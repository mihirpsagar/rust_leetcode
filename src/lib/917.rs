// Time taken: 00:41, 00:45 -> Wrong, 00:52 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut left = 0;
        let mut right = s.len() - 1;
        let mut s = s.chars().collect::<Vec<char>>();

        'outer: while left < right {
            while !s[left].is_ascii_alphabetic() {
                left += 1;
                if left == s.len() {
                    break 'outer;
                }
            }

            while !s[right].is_ascii_alphabetic() {
                right -= 1;
                if right == 0 {
                    break 'outer;
                }
            }

            if left < right {
                let tmp = s[left];
                s[left] = s[right];
                s[right] = tmp;
            }

            left += 1;
            right -= 1;

            if left == s.len() || right == 0 {
                break;
            }
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!"
        );
        assert_eq!(Solution::reverse_only_letters("7_28]".to_string()), "7_28]");
    }
}
