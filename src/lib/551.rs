// Time taken: 18:58, 19:01 -> Acc
struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a = 0;
        let mut l = 0;
        for ch in s.chars() {
            match ch {
                'A' => {
                    a += 1;
                    l = 0;
                    if a == 2 {
                        return false;
                    }
                }
                'L' => {
                    l += 1;
                    if l == 3 {
                        return false;
                    }
                }
                _ => {
                    l = 0;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_record("PPALLP".to_string()), true);
        assert_eq!(Solution::check_record("PPALLL".to_string()), false);
    }
}
