// Time taken: 11:57, 12:16 -> Acc
struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut result = (0, 0); // (Don't change 1st, Don't change 2nd)
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut found = false;

        while idx < s.len() - 1 {
            if s[idx] == s[idx + 1] {
                found = true;
                break;
            }
            idx += 1;
        }

        if found {
            Self::calculate_change(&s, idx, &mut result.0);
            Self::calculate_change(&s, idx + 1, &mut result.1);
        }

        return std::cmp::min(result.0, result.1);
    }

    pub fn calculate_change(arr: &Vec<char>, idx: usize, result: &mut i32) {
        let mut curr = arr[idx];
        let mut idx2 = idx + 1;

        while idx2 < arr.len() {
            if arr[idx2] == curr {
                *result += 1;
            }

            if curr == '0' {
                curr = '1';
            } else {
                curr = '0';
            }

            idx2 += 1;
        }

        if idx != 0 {
            idx2 = idx - 1;
            curr = arr[idx];
            loop {
                if arr[idx2] == curr {
                    *result += 1;
                }

                if curr == '0' {
                    curr = '1';
                } else {
                    curr = '0';
                }

                if idx2 == 0 {
                    break;
                }
                idx2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
        assert_eq!(Solution::min_operations("10".to_string()), 0);
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }
}
