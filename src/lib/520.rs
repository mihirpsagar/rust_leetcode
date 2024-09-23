// Time taken: 23:52, 00:01 -> Acc, 00:05 -> Optimized
struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let is_all_uppercase = word.chars().all(|x| x.is_uppercase());
        let is_all_lowercase = word.chars().all(|x| x.is_lowercase());
        let is_pascalcase = word.chars().nth(0).unwrap().is_uppercase()
            && word.chars().skip(1).all(|x| x.is_lowercase());

        return is_all_uppercase || is_all_lowercase || is_pascalcase;
    }

    // pub fn detect_capital_use(word: String) -> bool {
    //     let word = word.chars().collect::<Vec<char>>();

    //     if word[0].is_ascii_uppercase() {
    //         let mut case = None;
    //         for ch in word.iter().skip(1) {
    //             if let Some(val) = case {
    //                 if val == 1 {
    //                     if ch.is_ascii_lowercase() {
    //                         return false;
    //                     }
    //                 } else {
    //                     if ch.is_ascii_uppercase() {
    //                         return false;
    //                     }
    //                 }
    //             } else {
    //                 if ch.is_ascii_uppercase() {
    //                     case = Some(1);
    //                 } else {
    //                     case = Some(2);
    //                 }
    //             }
    //         }
    //     } else {
    //         for ch in word.iter().skip(1) {
    //             if ch.is_ascii_uppercase() {
    //                 return false;
    //             }
    //         }
    //     }

    //     return true;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
        assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
        assert_eq!(Solution::detect_capital_use("Google".to_string()), true);
    }
}
