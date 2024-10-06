// Time taken: 20:16, 20:29, 20:47 -> Acc
struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut larger = str1.chars().collect::<Vec<char>>();
        let mut smaller = str2.chars().collect::<Vec<char>>();
        let mut common_idx = 0;

        if larger[0] != smaller[0] {
            return String::new();
        }

        if str2.len() > str1.len() {
            let tmp = larger;
            larger = smaller;
            smaller = tmp;
        }

        let lcs = Self::lcs(&smaller);
        let mut idx1 = 0;
        let mut idx2 = 0;

        while (larger.len() - idx1) >= (smaller.len() - idx2) {
            if larger[idx1] == smaller[idx2] {
                idx1 += 1;
                idx2 += 1;
                if idx2 > common_idx {
                    common_idx = idx2;
                }
                if idx2 == smaller.len() {
                    break;
                }
            } else {
                if idx2 != 0 {
                    idx2 = lcs[idx2 - 1];
                } else {
                    idx2 = 0;
                    idx1 += 1;
                }
            }
        }

        let mut idx = common_idx;
        loop {
            if larger.len() % idx == 0 && smaller.len() % idx == 0 {
                let mut result = String::new();
                let mut idx2 = 0;
                while idx2 < idx {
                    result += &smaller[idx2].to_string();
                    idx2 += 1;
                }
                if Self::compare(&larger, &result) && Self::compare(&smaller, &result) {
                    return result;
                }
            }

            idx -= 1;
            if idx == 0 {
                break;
            }
        }

        return String::new();
    }

    fn compare(str1: &Vec<char>, str2: &String) -> bool {
        let str2 = str2.chars().collect::<Vec<char>>();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < str1.len() {
            if str1[idx1] != str2[idx2] {
                return false;
            }
            idx1 += 1;
            idx2 = (idx2 + 1) % str2.len();
        }

        return true;
    }

    fn lcs(str: &Vec<char>) -> Vec<usize> {
        let mut left = 0;
        let mut lcs = vec![0; str.len()];
        let mut idx = 1;

        while idx < str.len() {
            if str[idx] == str[left] {
                left += 1;
                lcs[idx] = left;
                idx += 1;
            } else {
                if left != 0 {
                    left = lcs[left - 1];
                } else {
                    left = 0;
                    lcs[idx] = 0;
                    idx += 1;
                }
            }
        }

        return lcs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            ""
        );
    }
}
