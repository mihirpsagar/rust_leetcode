// Time taken: 20:10, 21:09 -> Acc, 09:07 -> Wrong, 09:09 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let word = word.chars().collect::<Vec<char>>();
        let mut result = 0;
        let mut left = 0;
        // let mut right = 0;
        let mut min_ptr = 0;
        let mut idx = 0;
        let mut vowels = (0, 0, 0, 0, 0);

        while idx < word.len() {
            match word[idx] {
                'a' => {
                    vowels.0 += 1;
                    // right = idx;
                    if Self::is_all_vowels_present(&vowels) {
                        Self::minimize_ptr(&word, &mut min_ptr, &mut vowels);
                        result = result + (min_ptr - left + 1);
                    }
                }
                'e' => {
                    vowels.1 += 1;
                    // right = idx;
                    if Self::is_all_vowels_present(&vowels) {
                        Self::minimize_ptr(&word, &mut min_ptr, &mut vowels);
                        result = result + (min_ptr - left + 1);
                    }
                }
                'i' => {
                    vowels.2 += 1;
                    // right = idx;
                    if Self::is_all_vowels_present(&vowels) {
                        Self::minimize_ptr(&word, &mut min_ptr, &mut vowels);
                        result = result + (min_ptr - left + 1);
                    }
                }
                'o' => {
                    vowels.3 += 1;
                    // right = idx;
                    if Self::is_all_vowels_present(&vowels) {
                        Self::minimize_ptr(&word, &mut min_ptr, &mut vowels);
                        result = result + (min_ptr - left + 1);
                    }
                }
                'u' => {
                    vowels.4 += 1;
                    // right = idx;
                    if Self::is_all_vowels_present(&vowels) {
                        Self::minimize_ptr(&word, &mut min_ptr, &mut vowels);
                        result = result + (min_ptr - left + 1);
                    }
                }
                _ => {
                    left = idx + 1;
                    min_ptr = idx + 1;
                    // right = idx + 1;
                    vowels = (0, 0, 0, 0, 0);
                }
            }
            idx += 1;
        }

        return result as i32;
    }

    pub fn minimize_ptr(arr: &Vec<char>, ptr: &mut usize, vowels: &mut (i32, i32, i32, i32, i32)) {
        loop {
            match arr[*ptr] {
                'a' => {
                    if vowels.0 > 1 {
                        vowels.0 -= 1;
                        *ptr += 1;
                    } else {
                        break;
                    }
                }
                'e' => {
                    if vowels.1 > 1 {
                        vowels.1 -= 1;
                        *ptr += 1;
                    } else {
                        break;
                    }
                }
                'i' => {
                    if vowels.2 > 1 {
                        vowels.2 -= 1;
                        *ptr += 1;
                    } else {
                        break;
                    }
                }
                'o' => {
                    if vowels.3 > 1 {
                        vowels.3 -= 1;
                        *ptr += 1;
                    } else {
                        break;
                    }
                }
                'u' => {
                    if vowels.4 > 1 {
                        vowels.4 -= 1;
                        *ptr += 1;
                    } else {
                        break;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    // pub fn count_vowel_substrings(word: String) -> i32 {
    //     let mut vowel_count = (0, 0, 0, 0, 0);
    //     let mut start = None;
    //     let mut end = None;
    //     let mut result = 0;

    //     for (idx, ch) in word.chars().enumerate() {
    //         match ch {
    //             'a' => {
    //                 vowel_count.0 += 1;
    //                 if start.is_none() {
    //                     start = Some(idx);
    //                 }
    //                 end = Some(idx);
    //             }
    //             'e' => {
    //                 vowel_count.1 += 1;
    //                 if start.is_none() {
    //                     start = Some(idx);
    //                 }
    //                 end = Some(idx);
    //             }
    //             'i' => {
    //                 vowel_count.2 += 1;
    //                 if start.is_none() {
    //                     start = Some(idx);
    //                 }
    //                 end = Some(idx);
    //             }
    //             'o' => {
    //                 vowel_count.3 += 1;
    //                 if start.is_none() {
    //                     start = Some(idx);
    //                 }
    //                 end = Some(idx);
    //             }
    //             'u' => {
    //                 vowel_count.4 += 1;
    //                 if start.is_none() {
    //                     start = Some(idx);
    //                 }
    //                 end = Some(idx);
    //             }
    //             _ => {
    //                 if Self::is_all_vowels_present(&vowel_count) {
    //                     result += Self::count_substring(&word[start.unwrap()..=end.unwrap()]);
    //                 }
    //                 vowel_count = (0, 0, 0, 0, 0);
    //                 start = None;
    //                 end = None;
    //             }
    //         }
    //     }

    //     if Self::is_all_vowels_present(&vowel_count) {
    //         result += Self::count_substring(&word[start.unwrap()..=end.unwrap()]);
    //     }

    //     return result;
    // }

    // pub fn is_valid_substring(word: &[char]) -> bool {
    //     let mut vowels = (0, 0, 0, 0, 0);
    //     let mut idx = 0;
    //     while idx < word.len() {
    //         match word[idx] {
    //             'a' => {
    //                 vowels.0 += 1;
    //             }
    //             'e' => {
    //                 vowels.1 += 1;
    //             }
    //             'i' => {
    //                 vowels.2 += 1;
    //             }
    //             'o' => {
    //                 vowels.3 += 1;
    //             }
    //             'u' => {
    //                 vowels.4 += 1;
    //             }
    //             _ => {
    //                 return false;
    //             }
    //         }
    //         idx += 1;
    //     }

    //     return Self::is_all_vowels_present(&vowels);
    // }

    // pub fn count_substring(word: &str) -> i32 {
    //     let word = word.chars().collect::<Vec<char>>();
    //     let mut result = 0;
    //     let mut idx1 = 0;

    //     while idx1 < word.len() {
    //         let mut idx2 = idx1 + 4;
    //         while idx2 < word.len() {
    //             if Self::is_valid_substring(&word[idx1..=idx2]) {
    //                 result = result + (word.len() - idx2);
    //                 break;
    //             }
    //             idx2 += 1;
    //         }
    //         idx1 += 1;
    //     }

    //     return result as i32;
    // }

    pub fn is_all_vowels_present(vowel_count: &(i32, i32, i32, i32, i32)) -> bool {
        return vowel_count.0 != 0
            && vowel_count.1 != 0
            && vowel_count.2 != 0
            && vowel_count.3 != 0
            && vowel_count.4 != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
        assert_eq!(
            Solution::count_vowel_substrings("unicornarihan".to_string()),
            0
        );
        assert_eq!(
            Solution::count_vowel_substrings("cuaieuouac".to_string()),
            7
        );
        assert_eq!(
            Solution::count_vowel_substrings("bbaeixoubb".to_string()),
            0
        );
    }
}
