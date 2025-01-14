pub fn generate_lps(str: &Vec<char>) -> Vec<usize> {
    let mut lps = vec![0; str.len()];
    let mut len = 0;
    let mut idx = 1;

    while idx < str.len() {
        if str[idx] == str[len] {
            len += 1;
            lps[idx] = len;
            idx += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                idx += 1;
            }
        }
    }

    return lps;
}

pub fn get_all_matches(str1: String, str2: String) -> Vec<usize> {
    let mut result = Vec::new();
    let str1 = str1.chars().collect::<Vec<char>>();
    let str2 = str2.chars().collect::<Vec<char>>();
    let lps = generate_lps(&str2);
    let len1 = str1.len();
    let len2 = str2.len();
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < len1 && idx2 < len2 && (len1 - idx1) >= (len2 - idx2) {
        if str1[idx1] == str2[idx2] {
            idx1 += 1;
            idx2 += 1;

            if idx2 == len2 {
                result.push(idx1 - idx2);
                idx2 = lps[idx2 - 1];
            }
        } else {
            if idx2 != 0 {
                idx2 = lps[idx2 - 1];
            } else {
                idx1 += 1;
            }
        }
    }

    return result;
}

#[cfg(test)]
pub mod tests {
    use crate::dsa::kmp::get_all_matches;

    #[test]
    pub fn test_kmp() {
        assert_eq!(
            get_all_matches("abcab".to_string(), "ab".to_string()),
            vec![0, 3]
        );
        assert_eq!(
            get_all_matches("aabaacaadaabaaba".to_string(), "aaba".to_string()),
            vec![0, 9, 12]
        );
        assert_eq!(
            get_all_matches("aaaaaaaaa".to_string(), "aaa".to_string()),
            vec![0, 1, 2, 3, 4, 5, 6]
        );
    }
}
